use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{state::JSONStateManager, ws::Connections};

pub mod error;
pub mod state;
pub mod ws;

pub struct ScoreboardState {
    pub connections: Arc<Mutex<Connections>>,
    pub state_manager: Arc<Mutex<JSONStateManager>>,
}

impl ScoreboardState {
    pub fn new() -> Self {
        let connections = Arc::new(Mutex::new(Connections::default()));
        ScoreboardState {
            connections: connections.clone(),
            state_manager: Arc::new(Mutex::new(JSONStateManager::new(connections))),
        }
    }
}
