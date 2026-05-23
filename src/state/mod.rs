use std::sync::Arc;

use tokio::sync::Mutex;

mod listener;
mod manager;
pub mod path_trie;
pub mod state_trie;

pub use listener::JSONStateListener;
pub use manager::JSONStateManager;
pub use path_trie::PathTrie;
pub use state_trie::StateTrie;

use crate::ws::Connections;

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
