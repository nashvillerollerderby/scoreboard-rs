use chrono::Local;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::sleep;

const CLOCK_UPDATE_INTERVAL: u64 = 200;

struct ClockState {
    offset: i64,
    current_time: i64,
    stop_counter: i32,
    last_rewind: i64,
    clients: Vec<Arc<dyn ScoreBoardClockClient>>,
}

unsafe impl Send for ClockState {}

impl ClockState {
    pub fn rewind_to(&mut self, time: i64) {
        self.last_rewind = self.current_time - time;
        self.offset -= self.last_rewind;
    }

    pub fn advance(&mut self, ms: i64) {
        self.current_time += ms;
        self.update_clients();
    }

    pub fn stop(&mut self) {
        self.update_time();
        self.stop_counter += 1;
    }

    pub fn start(&mut self, do_catch_up: bool) {
        if !do_catch_up {
            self.offset = Local::now().timestamp_millis() - self.current_time;
        }
        self.stop_counter -= 1;
    }

    pub fn is_running(&self) -> bool {
        self.stop_counter == 0
    }

    pub fn register_client(&mut self, client: Arc<dyn ScoreBoardClockClient>) {
        self.clients.push(client);
    }

    fn update_time(&mut self) {
        if self.is_running() {
            self.current_time = Local::now().timestamp_millis() - self.offset;
        }
        self.update_clients();
    }

    fn update_clients(&self) {
        for client in &self.clients {
            client.update_time(self.current_time);
        }
    }
}

pub struct ScoreBoardClock {
    state: Arc<Mutex<ClockState>>,
    timer_thread: JoinHandle<()>,
}

impl ScoreBoardClock {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(ClockState {
            offset: Local::now().timestamp_millis(),
            stop_counter: 0,
            last_rewind: 0,
            current_time: 0,
            clients: vec![],
        }));
        let thread_state = state.clone();
        Self {
            state,
            timer_thread: spawn(async move {
                loop {
                    sleep(Duration::from_millis(CLOCK_UPDATE_INTERVAL / 4)).await;
                    let s = thread_state.clone();
                    spawn(async move {
                        let mut lock = s.lock().expect("unable to lock ScoreBoard clock state");
                        if lock.is_running() {
                            lock.update_time();
                        }
                    });
                }
            }),
        }
    }

    pub fn get_current_time(&self) -> i64 {
        let lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.current_time
    }

    pub fn get_current_walltime(&self) -> i64 {
        Local::now().timestamp_millis()
    }

    pub fn get_last_rewind(&self) -> i64 {
        let lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.last_rewind
    }

    pub fn rewind_to(&self, time: i64) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.rewind_to(time);
    }

    pub fn advance(&self, ms: i64) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.advance(ms);
    }

    pub fn stop(&self) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.stop();
    }

    pub fn start(&self, do_catch_up: bool) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.start(do_catch_up);
    }

    pub fn is_running(&self) -> bool {
        let lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.is_running()
    }

    pub fn register_client(&self, client: Arc<dyn ScoreBoardClockClient>) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.register_client(client.clone());
    }

    fn update_time(&self) {
        let mut lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.update_time()
    }

    fn update_clients(&self) {
        let lock = self
            .state
            .lock()
            .expect("unable to lock ScoreBoard clock state");
        lock.update_clients();
    }
}

pub trait ScoreBoardClockClient: Send {
    fn update_time(&self, ms: i64);
}

#[tokio::test]
async fn test_clock() {
    let clock = ScoreBoardClock::new();
    let time = clock.get_current_time();
    sleep(Duration::from_secs(1)).await;
    assert!(
        time + 950 < clock.get_current_time(),
        "Time {} !< {}",
        time + 950,
        clock.get_current_time()
    );
}

#[tokio::test]
async fn test_clock_client() {
    struct TestClient {
        times: Mutex<Vec<i64>>,
    }

    impl ScoreBoardClockClient for TestClient {
        fn update_time(&self, ms: i64) {
            self.times.lock().unwrap().push(ms);
        }
    }

    let client = Arc::new(TestClient {
        times: Mutex::new(vec![]),
    });
    let clock = ScoreBoardClock::new();
    clock.register_client(client.clone());
    sleep(Duration::from_millis(250)).await;
    clock.stop();
    sleep(Duration::from_millis(250)).await;
    assert_eq!(5, client.times.lock().unwrap().len())
}
