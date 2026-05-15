use envlayer::env_tags::EnvTags;
use envlayer::tag_filter::TagFilter;
use std::collections::HashMap;

#[test]
fn test_tag_and_has_tag() {
    let mut tags = EnvTags::new();
    tags.tag("DATABASE_URL", "secret");
    tags.tag("DATABASE_URL", "required");
    assert!(tags.has_tag("DATABASE_URL", "secret"));
    assert!(tags.has_tag("DATABASE_URL", "required"));
    assert!(!tags.has_tag("DATABASE_URL", "optional"));
}

#[test]
fn test_untag() {
    let mut tags = EnvTags::new();
    tags.tag("API_KEY", "secret");
    assert!(tags.untag("API_KEY", "secret"));
    assert!(!tags.has_tag("API_KEY", "secret"));
    assert!(!tags.untag("API_KEY", "nonexistent"));
}

#[test]
fn test_keys_with_tag() {
    let mut tags = EnvTags::new();
    tags.tag("API_KEY", "secret");
    tags.tag("DB_PASS", "secret");
    tags.tag("PORT", "optional");
    let mut keys = tags.keys_with_tag("secret");
    keys.sort();
    assert_eq!(keys, vec!["API_KEY", "DB_PASS"]);
}

#[test]
fn test_tags_for_key() {
    let mut tags = EnvTags::new();
    tags.tag("HOST", "required");
    tags.tag("HOST", "network");
    let t = tags.tags_for("HOST").unwrap();
    assert!(t.contains("required"));
    assert!(t.contains("network"));
}

#[test]
fn test_clear_key_ok() {
    let mut tags = EnvTags::new();
    tags.tag("TIMEOUT", "optional");
    assert!(tags.clear_key("TIMEOUT").is_ok());
    assert!(tags.tags_for("TIMEOUT").is_none());
}

#[test]
fn test_clear_key_missing() {
    let mut tags = EnvTags::new();
    assert!(tags.clear_key("MISSING").is_err());
}

#[test]
fn test_all_tags() {
    let mut tags = EnvTags::new();
    tags.tag("A", "secret");
    tags.tag("B", "required");
    tags.tag("C", "secret");
    let all = tags.all_tags();
    assert!(all.contains("secret"));
    assert!(all.contains("required"));
    assert_eq!(all.len(), 2);
}

#[test]
fn test_filter_all() {
    let mut tags = EnvTags::new();
    tags.tag("API_KEY", "secret");
    tags.tag("API_KEY", "required");
    tags.tag("PORT", "required");

    let env: HashMap<String, String> = [
        ("API_KEY".to_string(), "abc".to_string()),
        ("PORT".to_string(), "8080".to_string()),
    ]
    .into();

    let filter = TagFilter::new(&tags);
    let result = filter.filter_all(&env, &["secret", "required"]);
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("API_KEY"));
}

#[test]
fn test_filter_any() {
    let mut tags = EnvTags::new();
    tags.tag("API_KEY", "secret");
    tags.tag("PORT", "optional");

    let env: HashMap<String, String> = [
        ("API_KEY".to_string(), "abc".to_string()),
        ("PORT".to_string(), "8080".to_string()),
        ("HOST".to_string(), "localhost".to_string()),
    ]
    .into();

    let filter = TagFilter::new(&tags);
    let result = filter.filter_any(&env, &["secret", "optional"]);
    assert_eq!(result.len(), 2);
    assert!(!result.contains_key("HOST"));
}

#[test]
fn test_exclude() {
    let mut tags = EnvTags::new();
    tags.tag("API_KEY", "secret");
    tags.tag("DB_PASS", "secret");

    let env: HashMap<String, String> = [
        ("API_KEY".to_string(), "abc".to_string()),
        ("DB_PASS".to_string(), "pass".to_string()),
        ("PORT".to_string(), "8080".to_string()),
    ]
    .into();

    let filter = TagFilter::new(&tags);
    let result = filter.exclude(&env, &["secret"]);
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("PORT"));
}
