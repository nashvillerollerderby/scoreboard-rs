use crate::ScoreboardState;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use crossbeam::channel;
use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};
use uuid::Uuid;

pub struct StateTrie {
    changes: HashMap<String, Value>,
}

pub trait JSONStateUpdate {
    async fn handle_updates(&self, state: StateTrie);
}

pub type Connections = HashMap<Uuid, Connection>;

impl JSONStateUpdate for Connections {
    async fn handle_updates(&self, _state_trie: StateTrie) {
        for (_, connection) in self {
            // connection.send_registered_changes(&state_trie);
        }
    }
}

pub struct Connection {
    pub sender: channel::Sender<SocketMessageSend>,
    pub registered_for: HashSet<String>,
}

impl Connection {
    pub fn new(sender: channel::Sender<SocketMessageSend>) -> Self {
        Connection {
            registered_for: Default::default(),
            sender,
        }
    }
}

#[derive(Debug)]
pub enum SocketMessageSend {
    Updates(HashMap<String, Value>),
    Pong,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameData {
    Team1: u64,
    Team2: u64,
    Ruleset: u64,
    IntermissionClock: i64,
    Advance: bool,
    TO1: u8,
    TO2: u8,
    OR1: u8,
    OR2: u8,
    Points1: u32,
    Points2: u32,
    Period: u8,
    Jam: u8,
    PeriodClock: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessageRecv {
    // TODO Ping should not be here, it is managed by WS itself
    Ping {},
    Register {
        paths: Vec<String>,
    },
    Set {
        key: String,
        value: Value,
        flag: String,
    },
    StartNewGame {
        data: GameData,
    },
}

impl TryFrom<Utf8Bytes> for SocketMessageRecv {
    type Error = crate::error::Error;

    fn try_from(msg: Utf8Bytes) -> Result<Self, Self::Error> {
        log::debug!("Message received: {}", &msg);
        Ok(serde_json::from_str(&msg)?)
    }
}

fn string_message(string: String) -> Message {
    Message::Text(Utf8Bytes::from(string))
}

pub(crate) async fn ws_handler(
    State(shared_state): State<Arc<ScoreboardState>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("`{}` at {} connected.", user_agent, addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, shared_state.clone()))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, shared_state: Arc<ScoreboardState>) {
    if socket
        .send(Message::Ping(axum::body::Bytes::copy_from_slice(b"")))
        .await
        .is_ok()
    {
        log::debug!("Pinged {who}...");
    } else {
        log::warn!("Could not ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    let uuid = Uuid::new_v4();
    let (tx, rx) = channel::unbounded::<SocketMessageSend>();
    {
        shared_state
            .connections
            .lock()
            .await
            .insert(uuid.clone(), Connection::new(tx));
    }

    {
        log::info!("Sending new subscriber message to client {}", uuid);
        let state = shared_state.state.lock().await;
        socket
            .send(string_message(
                serde_json::to_string(&json!({
                    "state": *state,
                }))
                .unwrap(),
            ))
            .await
            .expect("Unable to send new subscriber message");
        log::info!("New subscriber message: {:?}", state);
        log::info!("New subscriber message sent to client {}", uuid);
    }

    let recv_connections = shared_state.connections.clone();

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    let send_uuid = uuid.clone();
    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        loop {
            if let Ok(message) = rx.try_recv() {
                log::debug!("Sending message to socket client");
                let message = match message {
                    SocketMessageSend::Updates(map) => {
                        string_message(serde_json::to_string(&map).unwrap())
                    }
                    SocketMessageSend::Pong => {
                        Message::Pong(axum::body::Bytes::copy_from_slice(b""))
                    }
                };

                match sender.send(message).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!(
                            "Unable to send message from scoreboard to client {}: {}",
                            send_uuid,
                            e
                        );
                        break;
                    }
                }
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            }
        }
    });

    let recv_uuid = uuid.clone();
    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            log::debug!(
                r#"Message received from client {}: "{}""#,
                recv_uuid,
                msg.to_text().unwrap()
            );
            match msg {
                Message::Text(msg) => match SocketMessageRecv::try_from(msg.clone()) {
                    Ok(msg) => match msg {
                        SocketMessageRecv::Register { paths } => {
                            log::debug!("Register paths {:?}", paths);
                            let mut connections = recv_connections.lock().await;
                            let connection = connections.get_mut(&recv_uuid).unwrap();
                            for path in paths {
                                connection.registered_for.insert(path);
                            }
                        }
                        SocketMessageRecv::Set { key, value, flag } => {
                            log::debug!(
                                "Received Set request k: `{}` v: `{}` f: `{}`",
                                key,
                                value,
                                flag
                            );
                        }
                        SocketMessageRecv::Ping {} => log::debug!("-> Ping"),
                        SocketMessageRecv::StartNewGame { data } => log::debug!("{:?}", data),
                    },
                    Err(e) => {
                        log::error!(
                            "could not convert message to SocketMessage {}: {:?}",
                            e,
                            msg
                        );
                    }
                },
                Message::Binary(_bytes) => log::debug!("<- Binary"),
                Message::Ping(msg) => {
                    log::debug!("Received ping {:?}", msg);
                    let mut connections = recv_connections.lock().await;
                    let connection = connections.get_mut(&recv_uuid).unwrap();
                    if connection.sender.send(SocketMessageSend::Pong).is_err() {
                        log::error!("")
                    }
                }
                Message::Pong(msg) => log::debug!("<- Pong: {:?}", msg),
                Message::Close(_close_frame) => {
                    let mut connections = recv_connections.lock().await;
                    connections
                        .remove_entry(&recv_uuid)
                        .expect("The client has to be registered in the connections");
                    log::info!("Unregistered client {}", recv_uuid);
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => log::debug!("Receiver {} closed", uuid),
                Err(a) => log::warn!("Error sending messages {a:?}")
            }
            recv_task.abort();
            shared_state.connections.lock().await.remove(&uuid);
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) => log::debug!("Sender {} closed", uuid),
                Err(b) => log::warn!("Error receiving messages {b:?}")
            }
            send_task.abort();
            shared_state.connections.lock().await.remove(&uuid);
        }
    }

    // returning from the handler closes the websocket connection
    log::debug!("Websocket context {who} destroyed");
}
