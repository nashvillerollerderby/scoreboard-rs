use std::{
    env::temp_dir,
    fs::{self, File},
    io::Write,
    path::Path,
};

use prometheus::Histogram;

use crate::{
    error::Error,
    state::{JSONStateListener, JSONStateManager, PathTrie, StateTrie},
};

pub struct JSONStateSnapshotter {
    directory: Box<Path>,
    filename: String,
    write_on_next_update: bool,
    state: StateTrie,
    filters: PathTrie,
    use_metrics: bool,
    update_state_duration: Histogram,
}

impl JSONStateListener for JSONStateSnapshotter {
    async fn send_updates(&mut self, state: &StateTrie, _changes: &StateTrie) {
        self.state = state.clone();
        if self.write_on_next_update {
            self.write_file();
            self.write_on_next_update = false;
        }
    }
}

impl JSONStateSnapshotter {
    pub fn new(jsm: JSONStateManager, g: Game, use_metrics: bool) -> Self {
        todo!()
    }
    pub fn write_on_next_update(&mut self) {
        self.write_on_next_update = true;
    }
    pub fn set_filename(&mut self, new_name: String) {
        self.filename = new_name;
    }

    pub async fn write_file(&self) -> Result<(), Error> {
        let timer = match self.use_metrics {
            true => Some(Histogram::start_timer(&self.update_state_duration)),
            false => None,
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
            // unwrap safety: if use metrics is true, then this should never be none.
            timer.unwrap().observe_duration();
        }

        Ok(())
    }
}
