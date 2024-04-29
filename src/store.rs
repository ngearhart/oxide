
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::serialization::NULL;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn global_store_set(key: String, value: String) {
    let mut hashmap = HASHMAP.lock().unwrap();
    hashmap.insert(key, value);
}

pub fn global_store_get(key: String) -> String {
    let hashmap = HASHMAP.lock().unwrap();
    hashmap.get(&key).unwrap_or(&NULL.to_owned()).to_string()
}
