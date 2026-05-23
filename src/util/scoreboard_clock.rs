use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::time::interval;

const CLOCK_UPDATE_INTERVAL_MS: u64 = 200;
const DATE_FORMAT: &str = "yyyy-MM-dd'T'HH:mm:ss.SSSXXX";

fn system_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ScoreBoardClockClient {
    #[cfg(test)]
    Test,
}

#[derive(Clone)]
pub struct ScoreBoardClock {
    offset: Arc<Mutex<u128>>,
    current_time: Arc<Mutex<u128>>,
    stopped: Arc<Mutex<bool>>,
    _break: Arc<Mutex<bool>>,
    last_rewind: Arc<Mutex<u128>>,
    clients: Arc<Mutex<HashMap<ScoreBoardClockClient, UnboundedSender<u128>>>>,
}

impl ScoreBoardClock {
    pub async fn new() -> ScoreBoardClock {
        let system_time_millis = system_millis();
        let clock = ScoreBoardClock {
            offset: Arc::new(Mutex::new(system_time_millis)),
            current_time: Arc::new(Mutex::new(system_time_millis)),
            stopped: Arc::new(Mutex::new(false)),
            _break: Arc::new(Mutex::new(false)),
            last_rewind: Arc::new(Mutex::new(0)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        };

        let clock_clone = clock.clone();
        tokio::spawn(async move {
            log::debug!("Started thread");
            let mut interval = interval(Duration::from_millis(CLOCK_UPDATE_INTERVAL_MS / 4));
            while !*clock_clone._break.lock().await {
                interval.tick().await;
                log::debug!("Tick");
                clock_clone.update_time().await;
            }
        });

        clock
    }

    pub async fn update_time(&self) {
        if !*self.stopped.lock().await {
            let mut current_time_lock = self.current_time.lock().await;
            *current_time_lock = system_millis() - *self.offset.lock().await;
            self.update_clients(current_time_lock.clone()).await;
        }
    }

    pub async fn update_clients(&self, time: u128) {
        log::debug!("Updating clients");
        for (client, tx) in &*self.clients.lock().await {
            tx.send(time)
                .expect(&format!("Unable to send time update to client {client:?}"));
        }
    }

    pub async fn register_client(&self, client: ScoreBoardClockClient) -> UnboundedReceiver<u128> {
        let mut clients = self.clients.lock().await;
        let (tx, rx) = unbounded_channel();
        clients.entry(client).and_modify(|v| {}).or_insert(tx);
        rx
    }

    pub async fn is_running(&self) -> bool {
        !*self.stopped.lock().await
    }

    pub async fn start(&self, do_catch_up: bool) {
        if do_catch_up {
            *self.offset.lock().await = system_millis() - *self.current_time.lock().await;
        }
        *self.stopped.lock().await = false;
    }

    pub async fn stop(&self) {
        self.update_time().await;
        *self.stopped.lock().await = true;
    }

    pub async fn advance(&self, ms: u128) {
        let mut current_time_lock = self.current_time.lock().await;
        *current_time_lock += ms;
        self.update_clients(current_time_lock.clone()).await
    }

    pub async fn get_last_rewind(&self) -> u128 {
        self.last_rewind.lock().await.clone()
    }

    pub async fn rewind_to(&self, time: u128) {
        let mut last_rewind_lock = self.last_rewind.lock().await;
        *last_rewind_lock = *self.current_time.lock().await - time;
        *self.offset.lock().await -= *last_rewind_lock;
    }

    pub fn get_current_walltime() -> u128 {
        system_millis()
    }

    pub async fn get_current_time(&self) -> u128 {
        self.current_time.lock().await.clone()
    }

    pub fn get_local_time(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::logging::init_logging;

    #[tokio::test]
    async fn test_timer_works() {
        init_logging();

        let clock = ScoreBoardClock::new().await;
        let mut rx = clock.register_client(ScoreBoardClockClient::Test).await;
        loop {
            let s = rx.recv().await;
            if let Some(s) = s {
                if s >= 1000 {
                    break;
                }
                log::info!("{s}ms have passed");
            }
        }
    }
}
