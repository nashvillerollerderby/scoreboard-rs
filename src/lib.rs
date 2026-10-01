use clap::Parser;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{state::JSONStateManager, ws::Connections};

pub mod error;
pub mod event;
pub mod model;
pub mod penalties;
pub mod rule;
pub mod state;
pub mod utils;
pub mod ws;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show the GUI
    #[arg(long, short, default_value_t = false)]
    pub gui: bool,

    /// Port on which to bind the web server
    #[arg(long, short, default_value_t = 8000)]
    pub port: i32,

    /// Host address on which to bind the web server
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    /// Path for files to import
    #[arg(long, short)]
    pub import: Option<String>,

    /// Enable metrics
    #[arg(long, short, default_value_t = false)]
    pub metrics: bool,

    /// The frequency in seconds that the autosave is triggered
    #[arg(long)]
    pub autosave_frequency_s: Option<u32>,
}

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
