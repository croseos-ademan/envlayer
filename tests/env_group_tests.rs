use envlayer::env_group::EnvGroup;

#[test]
fn test_new_group_is_empty() {
    let group = EnvGroup::new("test");
    assert!(group.is_empty());
    assert_eq!(group.len(), 0);
    assert_eq!(group.name(), "test");
}

#[test]
fn test_insert_and_get() {
    let mut group = EnvGroup::new("app");
    group.insert("HOST", "localhost");
    group.insert("PORT", "8080");

    assert_eq!(group.get("HOST"), Some(&"localhost".to_string()));
    assert_eq!(group.get("PORT"), Some(&"8080".to_string()));
    assert_eq!(group.get("MISSING"), None);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_insert_chaining() {
    let mut group = EnvGroup::new("chained");
    group.insert("A", "1").insert("B", "2").insert("C", "3");
    assert_eq!(group.len(), 3);
}

#[test]
fn test_merge_overwrites_existing_keys() {
    let mut base = EnvGroup::new("base");
    base.insert("KEY", "original");
    base.insert("ONLY_BASE", "yes");

    let mut override_group = EnvGroup::new("override");
    override_group.insert("KEY", "overridden");
    override_group.insert("EXTRA", "extra_val");

    base.merge(&override_group);

    assert_eq!(base.get("KEY"), Some(&"overridden".to_string()));
    assert_eq!(base.get("ONLY_BASE"), Some(&"yes".to_string()));
    assert_eq!(base.get("EXTRA"), Some(&"extra_val".to_string()));
    assert_eq!(base.len(), 3);
}

#[test]
fn test_keys_with_prefix() {
    let mut group = EnvGroup::new("prefixed");
    group.insert("APP_HOST", "localhost");
    group.insert("APP_PORT", "8080");
    group.insert("DB_HOST", "db.local");

    let mut app_keys = group.keys_with_prefix("APP_");
    app_keys.sort();
    assert_eq!(app_keys.len(), 2);
    assert!(app_keys.iter().all(|k| k.starts_with("APP_")));
}

#[test]
fn test_validate_keys_valid() {
    let mut group = EnvGroup::new("valid");
    group.insert("VALID_KEY", "value");
    assert!(group.validate_keys().is_ok());
}

#[test]
fn test_validate_keys_rejects_key_with_equals() {
    let mut group = EnvGroup::new("bad");
    group.insert("BAD=KEY", "value");
    let result = group.validate_keys();
    assert!(result.is_err());
}

#[test]
fn test_vars_returns_all_entries() {
    let mut group = EnvGroup::new("all");
    group.insert("X", "1");
    group.insert("Y", "2");
    let vars = group.vars();
    assert_eq!(vars.len(), 2);
    assert_eq!(vars.get("X"), Some(&"1".to_string()));
}
