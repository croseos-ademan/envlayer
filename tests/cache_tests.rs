use envlayer::cache::{CacheEntry, EnvCache};
use std::time::Duration;
use std::thread;

#[test]
fn test_insert_and_get() {
    let mut cache = EnvCache::new();
    cache.insert("KEY", "value", None);
    assert_eq!(cache.get("KEY"), Some("value"));
}

#[test]
fn test_missing_key_returns_none() {
    let cache = EnvCache::new();
    assert!(cache.get("MISSING").is_none());
}

#[test]
fn test_ttl_expiry() {
    let mut cache = EnvCache::new();
    cache.insert("TTL_KEY", "temp", Some(Duration::from_millis(50)));
    assert_eq!(cache.get("TTL_KEY"), Some("temp"));
    thread::sleep(Duration::from_millis(100));
    assert!(cache.get("TTL_KEY").is_none());
}

#[test]
fn test_no_ttl_does_not_expire() {
    let mut cache = EnvCache::new();
    cache.insert("PERM", "forever", None);
    thread::sleep(Duration::from_millis(20));
    assert_eq!(cache.get("PERM"), Some("forever"));
}

#[test]
fn test_invalidate_removes_key() {
    let mut cache = EnvCache::new();
    cache.insert("K", "v", None);
    assert!(cache.invalidate("K"));
    assert!(cache.get("K").is_none());
}

#[test]
fn test_invalidate_missing_returns_false() {
    let mut cache = EnvCache::new();
    assert!(!cache.invalidate("NOPE"));
}

#[test]
fn test_evict_expired_removes_only_expired() {
    let mut cache = EnvCache::new();
    cache.insert("LONG", "keep", None);
    cache.insert("SHORT", "drop", Some(Duration::from_millis(30)));
    thread::sleep(Duration::from_millis(60));
    let removed = cache.evict_expired();
    assert_eq!(removed, 1);
    assert!(cache.get("LONG").is_some());
    assert!(cache.get("SHORT").is_none());
}

#[test]
fn test_clear_empties_cache() {
    let mut cache = EnvCache::new();
    cache.insert("A", "1", None);
    cache.insert("B", "2", None);
    cache.clear();
    assert!(cache.is_empty());
}

#[test]
fn test_cache_entry_is_expired() {
    let entry = CacheEntry::new("v", Some(Duration::from_millis(10)));
    thread::sleep(Duration::from_millis(30));
    assert!(entry.is_expired());
}
