use std::collections::{HashMap, HashSet};
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct StateTrie {
    content: HashMap<String, Value>,
    is_path: bool,
    value: Option<Value>,
    subtries: HashMap<String, StateTrie>,
    regex_0: Regex
}

impl StateTrie {
    pub fn new(content: HashMap<String, Value>) -> StateTrie {
        StateTrie {
            content,
            is_path: false,
            value: None,
            subtries: HashMap::new(),
            regex_0: Regex::new("(?=[.(])").unwrap()
        }
    }

    pub fn empty() -> StateTrie {
        StateTrie {
            content: HashMap::new(),
            is_path: false,
            value: None,
            subtries: HashMap::new(),
            regex_0: Regex::new("(?=[.(])").unwrap()
        }
    }

    pub fn clone_inner(&self, null_values: bool) -> StateTrie {
        let mut clone = StateTrie::empty();
        clone.is_path = self.is_path;
        clone.value = match null_values {
            true => None,
            false => self.value.clone(),
        };
        for key in self.subtries.keys() {
            clone.subtries.insert(key.clone(), self.subtries.get(key).unwrap().clone_inner(null_values));
        }
        clone
    }

    pub fn clone_nulled(&self) -> StateTrie {
        self.clone_inner(true)
    }

    pub fn get(&self, key: String) -> Option<Value> {
        let key_split = self.regex_0.split(&key).map(|s| s.to_string()).collect::<Vec<String>>();
        self.get_inner(&key_split, 0)
    }

    pub fn get_inner(&self, path: &[String], index: usize) -> Option<Value> {
        if index == path.len() {
            self.value.clone()
        } else if self.subtries.contains_key(&path[index]) {
            self.subtries.get(&path[index])?.get_inner(path, index + 1)
        } else {
            None
        }
    }

    pub fn get_all(&self, filter_secrets: bool) -> HashMap<String, Value> {
        let mut results = HashMap::new();
        self.fetch_all(&mut results, "".to_string(), filter_secrets);
        results
    }

    pub fn fetch_all(&self, results: &mut HashMap<String, Value>, prefix: String, filter_secrets: bool) {
        if self.is_path && (!filter_secrets || !prefix.ends_with("Secret")) && self.value.is_some() {
            results.insert(prefix.to_string(), self.value.clone().unwrap());
        }
        for key in self.subtries.keys() {
            self.subtries.get(key).unwrap().fetch_all(results, format!("{prefix}{key}"), filter_secrets);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.is_path && self.subtries.is_empty()
    }

    pub fn size(&self) -> i64 {
        let mut size = if self.is_path { 1 } else { 0 };
        for subtrie in self.subtries.values() {
            size += subtrie.size();
        }
        size
    }

    pub fn add_all(&mut self, content: HashMap<String, Option<Value>>) {
        for (k, _v) in content.clone() {
            self.add(k.clone(), content.get(&k).unwrap().clone());
        }
    }

    pub fn add(&mut self, path: String, obj: Option<Value>) {
        let split = self.regex_0.split(path.as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
        let mut head = self.clone();
        for v in split.into_iter() {
            if head.subtries.contains_key(&v) {
                head = head.subtries.get(&v).unwrap().clone();
            } else {
                let child = StateTrie::empty();
                head.subtries.insert(v, child.clone());
                head = child;
            }
        }
        head.is_path = true;
        head.value = obj;
        if head.value.is_none() {
            head.subtries.clear();
        }
    }

    pub fn remove(&mut self, key: String) {
        let key_split = self.regex_0.split(&key).map(|s| s.to_string()).collect::<Vec<String>>();
        self.remove_inner(&key_split, 0);
    }

    pub fn remove_inner(&mut self, path: &[String], index: usize) -> bool {
        if index == path.len() {
            self.value = None;
            self.is_path = false;
        } else if self.subtries.contains_key(&path[index]) {
            if self.subtries.get_mut(&path[index]).unwrap().remove_inner(path, index + 1) {
                self.subtries.remove(&path[index]);
            }
        }
        self.is_empty()
    }

    pub fn merge_cloned(&self, change_trie: &StateTrie) -> StateTrie {
        self.merge_inner_cloned(&change_trie, false)
    }

    pub fn merge_inner_cloned(&self, change_trie: &StateTrie, removing: bool) -> StateTrie {
        let mut removing = removing;
        let mut change_trie = change_trie.clone();
        let mut clone = StateTrie::empty();
        if change_trie.is_path {
            if self.is_path && self.value.eq(&change_trie.value) {
                change_trie.is_path = false;
                clone.value = self.value.clone();
                clone.is_path = true;
            } else {
                change_trie.is_path = self.is_path || change_trie.value.is_some();
                clone.value = change_trie.value;
                clone.is_path = clone.value.is_none();
                removing = removing || self.value.is_none();
            }
        } else {
            clone.value = match removing {
                true => None,
                false => self.value.clone(),
            };
            clone.is_path = self.is_path && !removing;
        }

        let mut all_keys = self.subtries.keys().cloned().collect::<HashSet<String>>();
        all_keys.extend(change_trie.subtries.keys().cloned());

        for key in all_keys {
            if change_trie.subtries.contains_key(&key) {
                if self.subtries.contains_key(&key) {
                    let mut subclone = self.subtries.get(&key).unwrap().merge_inner_cloned(change_trie.subtries.get(&key).unwrap(), removing);
                    if !subclone.is_empty() {
                        clone.subtries.insert(key.clone(), subclone);
                    }
                    if change_trie.subtries.get(&key).unwrap().is_empty() {
                        change_trie.subtries.remove(&key);
                    }
                } else {
                    let subclone = change_trie.subtries.get_mut(&key).unwrap().clean_clone();
                    if subclone.is_none() {
                        change_trie.subtries.remove(&key);
                    } else {
                        clone.subtries.insert(key.clone(), subclone.unwrap());
                    }
                }
            } else if removing {
                change_trie.subtries.insert(key.clone(), self.subtries.get(&key).unwrap().clone_nulled());
            } else {
                clone.subtries.insert(key.clone(), self.subtries.get(&key).unwrap().clone());
            }
        }

        clone
    }

    pub fn clean_clone(&mut self) -> Option<StateTrie> {
        if self.value.is_none() {
            self.is_path = false;
        }
        let mut clone = StateTrie::empty();
        clone.value = self.value.clone();
        clone.is_path = self.is_path;
        for (k, mut v) in self.subtries.clone() {
            let subtrie = v.clean_clone();
            if subtrie.is_some() {
                clone.subtries.insert(k, subtrie.unwrap());
            }
        }
        match self.is_empty() {
            true => None,
            false => Some(clone)
        }
    }

    pub fn filter(&self, filter: PathTrie, filter_secrets: bool) -> HashMap<String, Value> {
        filter.intersect(self, filter_secrets)
    }
}