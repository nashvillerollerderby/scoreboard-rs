use std::cell::LazyCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{interval, sleep};

const CLOCK_UPDATE_INTERVAL_MS: u64 = 200;
const DATE_FORMAT: &str = "yyyy-MM-dd'T'HH:mm:ss.SSSXXX";

fn system_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub trait ScoreBoardClockClient: Send + Sync {
    fn update_time(&self, ms: u128);
}

pub struct ScoreBoardClock {
    offset: Mutex<u128>,
    current_time: Mutex<u128>,
    stopped: AtomicBool,
    _break: AtomicBool,
    last_rewind: Mutex<u128>,
    clients: Mutex<Vec<Arc<dyn ScoreBoardClockClient>>>,
    join_handle: Mutex<Option<JoinHandle<()>>>,
}

impl ScoreBoardClock {
    pub async fn new() -> Arc<ScoreBoardClock> {
        let system_time_millis = system_millis();
        let clock = Arc::new(ScoreBoardClock {
            offset: Mutex::new(system_time_millis),
            current_time: Mutex::new(system_time_millis),
            stopped: AtomicBool::new(false),
            _break: AtomicBool::new(false),
            last_rewind: Mutex::new(0),
            clients: Mutex::new(Vec::new()),
            join_handle: Mutex::new(None),
        });

        let timer_clock = clock.clone();
        let join = tokio::spawn(async move {
            log::info!("Started thread");
            let mut interval = interval(Duration::from_millis(CLOCK_UPDATE_INTERVAL_MS / 4));
            while !timer_clock._break.load(Ordering::SeqCst) {
                interval.tick().await;
                log::info!("Tick");
                let update_clock = timer_clock.clone();
                tokio::spawn(async move {
                    update_clock.update_time().await;
                });
            }
        });
        *clock.join_handle.lock().await = Some(join);

        clock
    }

    pub async fn update_time(&self) {
        if !self.stopped.load(Ordering::SeqCst) {
            let mut current_time_lock = self.current_time.lock().await;
            *current_time_lock = system_millis() - *self.offset.lock().await;
            self.update_clients(current_time_lock.clone()).await;
        }
    }

    pub async fn update_clients(&self, time: u128) {
        log::info!("Updating clients");
        for client in &*self.clients.lock().await {
            client.update_time(time);
        }
    }

    pub async fn register_client(&self, client: Arc<dyn ScoreBoardClockClient>) {
        self.clients.lock().await.push(client);
    }

    pub fn is_running(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

    pub async fn start(&self, do_catch_up: bool) {
        if do_catch_up {
            *self.offset.lock().await = system_millis() - *self.current_time.lock().await;
        }
        self.stopped.fetch_and(false, Ordering::SeqCst);
    }

    pub async fn stop(&self) {
        self.update_time().await;
        self.stopped.fetch_and(true, Ordering::SeqCst);
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

    pub struct Time(std::sync::Mutex<u128>);

    impl ScoreBoardClockClient for Time {
        fn update_time(&self, ms: u128) {
            *self.0.lock().expect("Unable to lock time") = ms;
        }
    }

    #[tokio::test]
    async fn test_timer_works() {
        init_logging();
        let system_time = ScoreBoardClock::get_current_walltime();
        let time = Time(std::sync::Mutex::new(system_time));
        let time = Arc::new(time);

        let clock = ScoreBoardClock::new().await;
        clock.register_client(time.clone()).await;
        sleep(Duration::from_millis(1002)).await;
        log::info!("{}", time.0.lock().unwrap());
        let time_value = time.0.lock().unwrap().clone();
        assert!(time_value >= 1000 && time_value <= 1002);
    }
}
