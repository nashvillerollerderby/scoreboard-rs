use std::fs;
use std::path::{Path, PathBuf};
use axum::http::{Response, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::Router;
use axum::routing::get;
use serde_json::Value;
use tower_http::services::ServeDir;

//             let file_service = ServeDir::new(dir.clone());
//             let router = handle_directories_with_router(dir).fallback_service(file_service);
//             app = app.fallback_service(router);

fn directory_fallback(path: PathBuf) -> Response<String> {
    let files = fs::read_dir(path).unwrap();
    let mut values = Vec::new();
    for file in files {
        match file {
            Ok(file) => values.push(Value::from(file.file_name().to_str().unwrap())),
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
    let value = Value::Array(values);
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(value.to_string())
        .unwrap()
}

fn handle_directory(path: PathBuf) -> Router {
    let mut router = Router::new();
    let dir = fs::read_dir(path.clone()).unwrap();
    let mut has_index = false;
    for file in dir {
        match file {
            Ok(file) => {
                if file.metadata().unwrap().is_dir() {
                    router = router.nest_service(
                        &format!("/{}", file.file_name().to_str().unwrap()),
                        handle_directory(file.path()),
                    );
                } else {
                    if file.file_name() == "index.html" {
                        log::debug!("{:?} has index", path);
                        has_index = true;
                        continue;
                    }
                }
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
    if !has_index {
        router = router
            .route("/", get(directory_fallback(path.clone())))
            .fallback_service(ServeDir::new(path));
    } else {
        router = router.fallback_service(ServeDir::new(path));
    }
    router
}

pub fn handle_directories_with_router(dir: &String) -> Router {
    let mut router = Router::new();

    let path = Path::new(dir);
    if fs::metadata(path).unwrap().is_dir() {
        router = handle_directory(path.to_path_buf());
    }

    router
}