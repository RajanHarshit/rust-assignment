use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use super::entry::CacheEntry;

#[derive(Clone)]
pub struct InMemoryCache {
    data: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(HashMap::<String, CacheEntry>::new()));
        let cleanup_data = Arc::clone(&data);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            let mut map = cleanup_data.lock().unwrap();
            map.retain(|_, entry| !entry.is_expired());
        });

        Self { data }
    }

    pub fn set(&mut self, key: &str, value: Vec<u8>, ttl_secs: u64) {
        let mut map = self.data.lock().unwrap();
        map.insert(key.to_string(), CacheEntry::new(value, ttl_secs));
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let map = self.data.lock().unwrap();
        map.get(key)
            .filter(|entry| !entry.is_expired())
            .map(|e| e.value.clone())
    }
}

impl Drop for InMemoryCache {
    fn drop(&mut self) {
        println!("Dropping cache: Cleaning up {} entries...", self.data.lock().unwrap().len());
    }
}
