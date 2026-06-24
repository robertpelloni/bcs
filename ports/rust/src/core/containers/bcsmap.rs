use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::hash::Hash;

// BcsMap is a thread-safe generic hash map
pub struct BcsMap<K, V> {
    items: Arc<RwLock<HashMap<K, V>>>,
}

impl<K: Eq + Hash, V: Clone> BcsMap<K, V> {
    pub fn new() -> Self {
        BcsMap {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut map = self.items.write().unwrap();
        map.insert(key, value);
    }

    pub fn value(&self, key: &K) -> Option<V> {
        let map = self.items.read().unwrap();
        map.get(key).cloned()
    }

    pub fn contains(&self, key: &K) -> bool {
        let map = self.items.read().unwrap();
        map.contains_key(key)
    }
}
