use std::sync::{LazyLock, Mutex};

const LOG4RS_DEFAULT_CONFIG: &str = include_str!("../log4rs.yaml");

static INITIALIZED: Mutex<bool> = Mutex::new(false);

/// Initializes logging from `./log4rs.yaml`, or from the default `log4rs.yaml` in the
/// [`apex-jump` repository](https://github.com/nashvillerollerderby/apex-jump/blob/main/log4rs.yaml).
pub fn init_logging() {
    if *INITIALIZED.lock().expect("Unable to lock INITIALIZED") {
        return;
    }
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(_) => {
            log::info!("Logging has been INITIALIZED.");
            *INITIALIZED.lock().expect("Unable to lock INITIALIZED") = true;
        }
        Err(_) => {
            let config =
                serde_yaml_ng::from_str::<log4rs::config::RawConfig>(LOG4RS_DEFAULT_CONFIG)
                    .unwrap();
            log4rs::init_raw_config(config).expect("Unable to initialize with default config.");
            log::info!(
                "No log4rs.yaml file found in working directory. Using default config (stdout)."
            );
        }
    }
}
