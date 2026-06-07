use std::{
    env::temp_dir,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

use log::warn;
use prometheus::{Histogram, HistogramOpts};

use crate::{
    error::Error,
    model::Game,
    state::{JSONStateListener, JSONStateManager, PathTrie, StateTrie},
    utils::base_path::BasePath,
};

pub struct JSONStateSnapshotter {
    directory: PathBuf,
    filename: String,
    write_on_next_update: AtomicBool,
    state: StateTrie,
    filters: PathTrie,
    use_metrics: bool,
    update_state_duration: Option<Histogram>,
}

impl JSONStateListener for JSONStateSnapshotter {
    async fn send_updates(&mut self, state: &StateTrie, _changes: &StateTrie) {
        self.state = state.clone();

        if self.write_on_next_update.swap(false, Ordering::SeqCst) {
            let _ = self.write_file().await;
        }
    }
}

impl JSONStateSnapshotter {
    pub fn new(jsm: JSONStateManager, g: Game, use_metrics: bool) -> Self {
        let mut snapshotter = JSONStateSnapshotter {
            directory: BasePath::get(),
            filename: g.filename,
            write_on_next_update: AtomicBool::new(false),
            state: StateTrie::empty(),
            filters: PathTrie::empty(),
            use_metrics,
            update_state_duration: None,
        };

        if snapshotter.use_metrics && snapshotter.update_state_duration.is_none() {
            let opts = HistogramOpts::new(
                "crg_json_state_disk_snapshot_duration_seconds",
                "Time spent writing JSON state snapshots to disk",
            );

            let histogram = match Histogram::with_opts(opts) {
                Ok(h) => Some(h),
                Err(e) => {
                    warn!(
                        "Failed to initialize Snapshotter Metrics, falling back on logging only: {e}"
                    );
                    None
                }
            };

            snapshotter.update_state_duration = histogram
        }

        snapshotter.filters.add("ScoreBoard.Version");
        snapshotter
            .filters
            .add(&format!("ScoreBoard.Game({})", g.id));

        snapshotter
    }

    pub fn write_on_next_update(&mut self) {
        self.write_on_next_update.store(true, Ordering::SeqCst);
    }
    pub fn set_filename(&mut self, new_name: String) {
        self.filename = new_name;
    }

    pub async fn write_file(&self) -> Result<(), Error> {
        let timer = match (self.use_metrics, self.update_state_duration.clone()) {
            (true, Some(histogram)) => Some(Histogram::start_timer(&histogram)),
            (_, _) => None,
        };

        let file_path = self
            .directory
            .join("html")
            .join("game-data")
            .join(format!("{}.json", self.filename));

        let prev_path = self
            .directory
            .join("html")
            .join("game-data")
            .join(format!("{}_prev.json", self.filename));

        let json = serde_json::to_string_pretty(&self.state.filter(self.filters.clone(), true))?;

        let temp_path = temp_dir().join(format!("{}.json", self.filename));
        let mut temp = File::create(&temp_path)?;

        match temp.write(json.as_bytes()) {
            Ok(_) => {
                fs::rename(&file_path, &prev_path)?;
                fs::rename(&temp_path, &file_path)?;
            }
            Err(e) => {
                return Err(Error::IO(e));
            }
        }

        if self.use_metrics {
            timer
                .expect("If use metrics is true, this should never be false")
                .observe_duration();
        }

        Ok(())
    }
}
