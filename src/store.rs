
use std::collections::{hash_map, HashMap};

// Wrapper around HashMap
pub struct Store<'a> {
    map: HashMap<&'a str, &'a str>
}

impl<'a> Store<'a> {
    pub fn new() -> Store<'a> {
        Store {
            map: HashMap::new()
        }
    }

    pub fn set(&mut self, key: &'a str, val: &'a str) {
        self.map.insert(&key, &val);
    }

    pub fn get(&mut self, key: &'a str) -> Option<&str> {
        self.map.get(&key).copied()
    }
}