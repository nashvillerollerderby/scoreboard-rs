use crate::state::StateTrie;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct PathTrie {}

impl PathTrie {
    pub fn new(content: HashSet<String>) -> PathTrie {
        PathTrie {}
    }

    pub fn empty() -> PathTrie {
        PathTrie {}
    }

    pub fn add_all(&mut self, content: HashSet<String>) {
        todo!()
    }

    pub fn add(&mut self, path: String) {
        todo!()
    }

    pub fn covers(&self, path: String) -> bool {
        todo!()
    }

    fn covers_inner(&self, path: String) -> bool {
        todo!()
    }

    pub fn merge(&mut self, other: PathTrie) {
        todo!()
    }

    pub fn intersect(
        &self,
        state_trie: &StateTrie,
        filter_secrets: bool,
    ) -> HashMap<String, Value> {
        let mut results = HashMap::new();
        self.intersect_inner(state_trie, &mut results, "".to_string(), filter_secrets);
        results
    }

    fn intersect_inner(
        &self,
        state_trie: &StateTrie,
        results: &mut HashMap<String, Value>,
        prefix: String,
        filter_secrets: bool,
    ) {
        todo!()
    }
}
