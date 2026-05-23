use crate::error::Result;
use crate::state::StateTrie;
use crate::state::listener::JSONStateListener;
use crate::ws::{Connection, Connections};
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

#[cfg(test)]
use std::time::Duration;
use tokio::sync::Mutex;

pub struct JSONStateManager {
    connections: Arc<Mutex<Connections>>,
    pub state: StateTrie,
    pending: Arc<AtomicI64>,
}

impl JSONStateManager {
    pub fn new(connections: Arc<Mutex<Connections>>) -> JSONStateManager {
        JSONStateManager {
            connections,
            state: StateTrie::empty(),
            pending: Arc::new(AtomicI64::new(0)),
        }
    }

    pub async fn register(&mut self, source: &mut Connection) {
        let local_state = self.state.clone();
        self.pending.fetch_add(1, Ordering::SeqCst);
        let pending = self.pending.clone();
        source.send_updates(&local_state, &local_state).await;
        pending.fetch_sub(1, Ordering::SeqCst);
    }

    pub async fn update_state(&mut self, key: String, value: Value) {
        let mut updates = StateTrie::empty();
        updates.add(key, Some(value));
        self.update_state_inner(updates).await;
    }

    pub async fn update_state_inner(&mut self, updates: StateTrie) {
        self.state = self.state.merge_cloned(&updates);
        if !updates.is_empty() {
            let local_changed = updates;

            let mut lock = self.connections.lock().await;
            for (_uuid, connection) in &mut *lock {
                self.pending.fetch_add(1, Ordering::SeqCst);
                connection.send_updates(&self.state, &local_changed).await;
                self.pending.fetch_sub(1, Ordering::SeqCst);
            }

            // TODO handle JSON state snapshotter
        }
    }

    #[cfg(test)]
    pub async fn wait_for_sent(&self) {
        while self.pending.load(Ordering::SeqCst) > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

#[cfg(test)]
#[allow(unstable_features)]
mod tests {
    use super::*;
    use crate::ws::{Connection, SocketMessageSend};
    use crossbeam::channel::Receiver;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn init() -> (JSONStateManager, Connection, Receiver<SocketMessageSend>) {
        let (tx, rx) = crossbeam::channel::unbounded();
        let connection = Connection::new(tx);
        let connections = Arc::new(Mutex::new(HashMap::<Uuid, Connection>::from_iter([(
            Uuid::new_v4(),
            connection.clone(),
        )])));
        let state_manager = JSONStateManager::new(connections);
        (state_manager, connection, rx)
    }

    #[tokio::test]
    async fn listener_gets_update_on_register() {
        let (mut state_manager, mut connection, rx) = init();

        state_manager
            .update_state("foo".to_string(), Value::String("bar".to_string()))
            .await;
        state_manager.register(&mut connection).await;

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => {
                assert!(matches!(msg, SocketMessageSend::Updates(..)));
                match msg {
                    SocketMessageSend::Updates(v) => {
                        assert_eq!(*v.get("foo").unwrap(), Value::from("bar"));
                    }
                    _ => panic!(),
                }
            }
            Err(_) => panic!("Timeout error waiting to receive socket message"),
        }
    }
}
