use envlayer::cache_policy::{CachePolicy, CachePolicyBuilder};
use envlayer::cached_resolver::CachedResolver;
use std::collections::HashMap;
use std::time::Duration;
use std::thread;

fn make_source() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("HOST".into(), "localhost".into());
    m.insert("PORT".into(), "8080".into());
    m
}

#[test]
fn test_resolve_present_key() {
    let mut r = CachedResolver::new(make_source(), CachePolicy::Forever);
    assert_eq!(r.resolve("HOST").unwrap(), "localhost");
}

#[test]
fn test_resolve_missing_key_errors() {
    let mut r = CachedResolver::new(make_source(), CachePolicy::Forever);
    assert!(r.resolve("MISSING").is_err());
}

#[test]
fn test_cache_populated_after_resolve() {
    let mut r = CachedResolver::new(make_source(), CachePolicy::Forever);
    r.resolve("PORT").unwrap();
    assert_eq!(r.cache_len(), 1);
}

#[test]
fn test_no_cache_policy_skips_cache() {
    let mut r = CachedResolver::new(make_source(), CachePolicy::NoCache);
    r.resolve("HOST").unwrap();
    assert_eq!(r.cache_len(), 0);
}

#[test]
fn test_invalidate_clears_entry() {
    let mut r = CachedResolver::new(make_source(), CachePolicy::Forever);
    r.resolve("HOST").unwrap();
    assert!(r.invalidate("HOST"));
    assert_eq!(r.cache_len(), 0);
}

#[test]
fn test_ttl_policy_expires() {
    let policy = CachePolicyBuilder::new().ttl_secs(0).build();
    let mut r = CachedResolver::new(make_source(), policy);
    r.resolve("HOST").unwrap();
    thread::sleep(Duration::from_millis(50));
    let evicted = r.evict_expired();
    assert!(evicted >= 1);
}

#[test]
fn test_policy_builder_no_cache() {
    let p = CachePolicyBuilder::new().no_cache().build();
    assert_eq!(p, CachePolicy::NoCache);
    assert!(!p.should_cache());
}

#[test]
fn test_policy_builder_forever() {
    let p = CachePolicyBuilder::new().forever().build();
    assert_eq!(p, CachePolicy::Forever);
    assert!(p.should_cache());
    assert!(p.to_ttl().is_none());
}
