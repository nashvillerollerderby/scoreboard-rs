use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use axum::{Json, Router};
use clap::Parser;
use local_ip_address::{local_ip, local_ipv6};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub(crate) mod event;
pub(crate) mod error;
mod logging;
pub(crate) mod model;
pub(crate) mod state;
mod static_files;
pub(crate) mod utils;
mod ws;

use crate::state::ScoreboardState;
use error::Result;
use scoreboard_rs::Args;
use static_files::handle_directories_with_router;
use ws::ws_handler;

pub const PENALTIES_RDCL: &str = include_str!("../config/penalties/RDCL.json");
pub const PENALTIES_WFTDA_2016: &str = include_str!("../config/penalties/wftda2016.json");
pub const PENALTIES_WFTDA_2018: &str = include_str!("../config/penalties/wftda2018.json");

pub async fn urls_route(State(shared_state): State<Arc<ScoreboardState>>) -> impl IntoResponse {
    urls(&shared_state.args)
}

fn urls(args: &Args) -> String {
    let port = args.port;

    let mut hosts = vec![
        format!("localhost:{}", port),
        format!("0.0.0.0:{}", port),
    ];

    if let Ok(ipv4) = local_ip() {
        hosts.push(format!("{}:{}", ipv4, port));
    } else {
        log::warn!("No IPV4 address found.");
    }
    if let Ok(ipv6) = local_ipv6() {
        hosts.push(format!("{}:{}", ipv6, port));
    } else {
        log::warn!("No IPV6 address found.");
    }

    hosts.join(",\n")
}

async fn shutdown(app_state: Arc<ScoreboardState>) {
    // TODO run autosave p2
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    logging::init_logging();

    let app_state = Arc::new(ScoreboardState::new(args.clone()));

    // TODO load version information p1

    if args.metrics {
        // TODO initialize metrics p3
    }

    // TODO handle autosave p2

    let app = Router::new()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .route("/WS/", any(ws_handler))
        .route("/urls", get(urls_route));

    // Set up static serve directory for webserver
    #[cfg(target_os = "windows")]
    let dir = r#"static\html"#.to_string();
    #[cfg(not(target_os = "windows"))]
    let dir = "static/html".to_string();
    let serve_dir = ServeDir::new(dir.clone());
    let files_router = handle_directories_with_router(&dir).fallback_service(serve_dir);
    let app = app.fallback_service(files_router);

    let app = app.with_state(app_state.clone());

    if args.gui {
        // TODO: init gui? p4
    }

    log::info!("Starting server on the following IPs:\n{}", urls(&args));
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    // .with_graceful_shutdown(shutdown(app_state))
    .await?;
    Ok(())
}
