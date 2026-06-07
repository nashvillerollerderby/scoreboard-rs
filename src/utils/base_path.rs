use std::{
    env,
    path::{Path, PathBuf},
    sync::OnceLock,
};

static BASE_PATH: OnceLock<PathBuf> = OnceLock::new();

pub struct BasePath {}

impl BasePath {
    pub fn get() -> PathBuf {
        BASE_PATH
            .clone()
            .get_or_init(|| env::current_dir().expect("Unable to fetch current directory"))
            .to_path_buf()
    }

    pub fn set(path: impl AsRef<Path>) -> Result<(), PathBuf> {
        BASE_PATH.set(path.as_ref().to_path_buf())
    }
}
