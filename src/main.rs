use axum::Router;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod error;
mod logging;
pub(crate) mod state;
mod static_files;
mod util;
mod ws;

use crate::state::JSONStateManager;
use error::Result;
use static_files::handle_directories_with_router;
use ws::{Connections, ws_handler};

pub type Version = HashMap<String, Option<Value>>;

pub const RELEASE: &str = include_str!("../release.json");

pub const PENALTIES_RDCL: &str = include_str!("../config/penalties/RDCL.json");
pub const PENALTIES_WFTDA_2016: &str = include_str!("../config/penalties/wftda2016.json");
pub const PENALTIES_WFTDA_2018: &str = include_str!("../config/penalties/wftda2018.json");

#[derive(Parser, Debug, Serialize, Deserialize)]
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

pub struct ScoreBoardState {
    pub connections: Arc<Mutex<Connections>>,
    pub state_manager: Arc<Mutex<JSONStateManager>>,
}

impl ScoreBoardState {
    pub fn new() -> Self {
        let connections = Arc::new(Mutex::new(Connections::default()));
        ScoreBoardState {
            connections: connections.clone(),
            state_manager: Arc::new(Mutex::new(JSONStateManager::new(connections))),
        }
    }
}

pub async fn urls() -> impl IntoResponse {
    "0.0.0.0:8000\nlocalhost:8000"
}

async fn shutdown(app_state: Arc<ScoreBoardState>) {
    // TODO run autosave p2
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    logging::init_logging();

    let app_state = Arc::new(ScoreBoardState::new());

    // load version information
    {
        let json = serde_json::from_str::<Version>(RELEASE)?;
        let mut state_manager = app_state.state_manager.lock().await;
        state_manager.state.add_all(json);
    }
    // TODO initialize JSON listener p1

    if args.metrics {
        // TODO initialize metrics p3
    }

    // TODO handle autosave p2

    let app = Router::new()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .route("/WS/", any(ws_handler))
        .route("/urls", get(urls));

    // Set up static serve directory for webserver
    let path = Path::new("static").join("html");
    let dir = path.to_str().unwrap().to_string();
    let serve_dir = ServeDir::new(dir.clone());
    let files_router = handle_directories_with_router(&dir).fallback_service(serve_dir);
    let app = app.fallback_service(files_router);

    let app = app.with_state(app_state.clone());

    if args.gui {
        // TODO: init gui? p4
    }

    log::info!("Starting server on {}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    // .with_graceful_shutdown(shutdown(app_state))
    .await?;
    Ok(())
}
