// unsure for now whether this is necessary since it just seems like
// a pointer to a file + getters/setters. Very javacore :tm:
// for now I'm choosing to leave it like this.

pub struct BasePath {
    base_path: Box<Path>,
}

impl BasePath {
    pub fn get(&self) -> Path {
        self.base_path
    }

    pub fn set(&self, path: String) {
        self.base_path = Path::new(path);
    }
}
