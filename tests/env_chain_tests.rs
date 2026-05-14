use std::collections::HashMap;
use envlayer::env_chain::EnvChain;
use envlayer::chain_builder::ChainBuilder;
use envlayer::layer::Layer;

fn make_layer(pairs: &[(&str, &str)]) -> Layer {
    let map: HashMap<String, String> = pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    Layer::from_map(map)
}

#[test]
test_chain_resolve_highest_priority_wins() {
    let mut chain = EnvChain::new();
    chain.push(10, make_layer(&[("APP_ENV", "staging")]));
    chain.push(20, make_layer(&[("APP_ENV", "production")]));
    assert_eq!(chain.resolve("APP_ENV"), Some("production".into()));
}

#[test]
fn test_chain_resolve_fallback_to_lower_priority() {
    let mut chain = EnvChain::new();
    chain.push(5, make_layer(&[("DB_HOST", "localhost")]));
    chain.push(15, make_layer(&[("APP_ENV", "ci")]));
    assert_eq!(chain.resolve("DB_HOST"), Some("localhost".into()));
}

#[test]
fn test_chain_resolve_missing_key() {
    let chain = EnvChain::new();
    assert_eq!(chain.resolve("MISSING"), None);
}

#[test]
fn test_chain_flatten_higher_priority_overwrites() {
    let mut chain = EnvChain::new();
    chain.push(1, make_layer(&[("KEY", "base"), ("ONLY_BASE", "yes")]));
    chain.push(2, make_layer(&[("KEY", "override")]));
    let flat = chain.flatten();
    assert_eq!(flat.get("KEY").map(String::as_str), Some("override"));
    assert_eq!(flat.get("ONLY_BASE").map(String::as_str), Some("yes"));
}

#[test]
fn test_chain_len_and_is_empty() {
    let mut chain = EnvChain::new();
    assert!(chain.is_empty());
    chain.push(1, make_layer(&[("X", "1")]));
    assert_eq!(chain.len(), 1);
}

#[test]
fn test_chain_remove_priority() {
    let mut chain = EnvChain::new();
    chain.push(10, make_layer(&[("A", "1")]));
    chain.push(20, make_layer(&[("A", "2")]));
    chain.remove_priority(20).unwrap();
    assert_eq!(chain.resolve("A"), Some("1".into()));
}

#[test]
fn test_chain_remove_priority_not_found() {
    let mut chain = EnvChain::new();
    chain.push(10, make_layer(&[("A", "1")]));
    assert!(chain.remove_priority(99).is_err());
}

#[test]
fn test_builder_with_map() {
    let mut map = HashMap::new();
    map.insert("CI".to_string(), "true".to_string());
    let chain = ChainBuilder::new().with_map(5, map).build();
    assert_eq!(chain.resolve("CI"), Some("true".into()));
}

#[test]
fn test_builder_with_dotenv_str() {
    let chain = ChainBuilder::new()
        .with_dotenv_str(10, "FOO=bar\nBAZ=qux")
        .unwrap()
        .build();
    assert_eq!(chain.resolve("FOO"), Some("bar".into()));
    assert_eq!(chain.resolve("BAZ"), Some("qux".into()));
}
