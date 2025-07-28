mod cache;

use cache::store::InMemoryCache;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut cache = InMemoryCache::new();

    cache.set("token", b"ABC123".to_vec(), 2); // expires in 2 seconds
    cache.set("user", b"Harshit".to_vec(), 5);  // expires in 5 seconds

    println!("Initial: {:?}", cache.get("token"));
    sleep(Duration::from_secs(3));
    println!("After 3s: {:?}", cache.get("token")); // should be expired

    println!("Still valid: {:?}", cache.get("user"));
}
