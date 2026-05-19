
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
    #[error("serde_json error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;