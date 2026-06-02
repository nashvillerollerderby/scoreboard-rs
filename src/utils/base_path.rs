// This seems like some kind of global variable kind of thing. I'm choosing to replace it with an environment variable
// in practice, this is a compatibility layer around get_current_dir and set_current_dir (mostly to preserve the API of the CRG scoreboard)
// to quote the original implementation
// it should be set once at the beginning of the program, and then never set again with the exception of unit tests.

use std::{env, path::PathBuf};
pub struct BasePath {}

impl BasePath {
    pub fn get() -> PathBuf {
        let key = "BASE_PATH";
        env::var(key)
            .map(PathBuf::from)
            .unwrap_or_else(|_| env::current_dir().expect("failed to get current dir"))
    }

    pub fn set(&mut self, path: String) {
        unsafe { env::set_var("BASE_PATH", path) };
    }
}
