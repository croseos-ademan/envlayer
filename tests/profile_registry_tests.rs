use envlayer::profile::Profile;
use envlayer::profile_registry::ProfileRegistry;
use envlayer::profile_loader::ProfileLoader;

fn make_profile(name: &str, key: &str, val: &str) -> Profile {
    let mut p = Profile::new(name);
    p.set(key, val);
    p
}

#[test]
fn test_register_and_get() {
    let mut reg = ProfileRegistry::new();
    reg.register(make_profile("local", "DEBUG", "true")).unwrap();
    assert!(reg.get("local").is_some());
    assert!(reg.get("ci").is_none());
}

#[test]
fn test_activate_and_active_profile() {
    let mut reg = ProfileRegistry::new();
    reg.register(make_profile("ci", "CI", "1")).unwrap();
    reg.activate("ci").unwrap();
    let active = reg.active_profile().unwrap();
    assert_eq!(active.name, "ci");
}

#[test]
fn test_activate_nonexistent_returns_error() {
    let mut reg = ProfileRegistry::new();
    let result = reg.activate("ghost");
    assert!(result.is_err());
}

#[test]
fn test_list_names_sorted() {
    let mut reg = ProfileRegistry::new();
    reg.register(make_profile("staging", "X", "1")).unwrap();
    reg.register(make_profile("ci", "X", "2")).unwrap();
    reg.register(make_profile("local", "X", "3")).unwrap();
    assert_eq!(reg.list_names(), vec!["ci", "local", "staging"]);
}

#[test]
fn test_remove_clears_active() {
    let mut reg = ProfileRegistry::new();
    reg.register(make_profile("local", "A", "b")).unwrap();
    reg.activate("local").unwrap();
    reg.remove("local");
    assert!(reg.active_profile().is_none());
    assert!(reg.is_empty());
}

#[test]
fn test_loader_from_triples() {
    let triples = vec![
        ("local", "DEBUG", "true"),
        ("local", "PORT", "8080"),
        ("ci", "CI", "1"),
    ];
    let profiles = ProfileLoader::from_triples(triples).unwrap();
    assert_eq!(profiles.len(), 2);
}

#[test]
fn test_loader_invalid_name() {
    let triples = vec![("bad name", "K", "v")];
    assert!(ProfileLoader::from_triples(triples).is_err());
}
