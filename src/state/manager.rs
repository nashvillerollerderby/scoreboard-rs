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
    state: StateTrie,
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

    pub async fn register(&mut self, source: &mut Connection) -> Result<()> {
        let local_state = self.state.clone();
        self.pending.fetch_add(1, Ordering::SeqCst);
        let pending = self.pending.clone();
        source.send_updates(&local_state, &local_state).await;
        pending.fetch_sub(1, Ordering::SeqCst);
        Ok(())
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
