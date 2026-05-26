use axum::Router;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod error;
mod logging;
mod model;
mod static_files;
mod ws;

use error::Result;
use static_files::handle_directories_with_router;
use ws::{Connections, ws_handler};

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

pub struct ScoreboardState {
    pub state: Arc<Mutex<HashMap<String, Value>>>,
    pub connections: Arc<Mutex<Connections>>,
}

impl ScoreboardState {
    pub fn new() -> Self {
        ScoreboardState {
            state: Default::default(),
            connections: Default::default(),
        }
    }
}

pub async fn urls() -> impl IntoResponse {
    "0.0.0.0:8000\nlocalhost:8000"
}

async fn shutdown(app_state: Arc<ScoreboardState>) {
    // TODO run autosave p1
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    logging::init_logging();

    // TODO load version information p1
    // TODO initialize JSON State manager p1
    // TODO initialize JSON listener p1

    if args.metrics {
        // TODO initialize metrics p3
    }

    // TODO handle autosave p1

    let app_state = Arc::new(ScoreboardState::new());

    let app = Router::new()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .route("/WS/", any(ws_handler))
        .route("/urls", get(urls));

    // Set up static serve directory for webserver
    let dir = "static/html".to_string();
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
