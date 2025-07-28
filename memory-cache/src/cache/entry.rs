use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CacheEntry {
    pub value: Vec<u8>,
    pub expires_at: Instant,
}

impl CacheEntry {
    pub fn new(value: Vec<u8>, ttl_secs: u64) -> Self {
        Self {
            value,
            expires_at: Instant::now() + Duration::from_secs(ttl_secs),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.value
    }
}
