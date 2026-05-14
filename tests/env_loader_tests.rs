use envlayer::env_loader::EnvLoader;
use envlayer::layer::Layer;
use std::collections::HashMap;

fn make_layer(vars: &[(&str, &str)]) -> Layer {
    let map: HashMap<String, String> = vars
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    Layer::from_map(map)
}

#[test]
fn test_load_single_layer() {
    let layer = make_layer(&[("APP_ENV", "development"), ("PORT", "8080")]);
    let loader = EnvLoader::new().with_layer(layer);
    let result = loader.load().unwrap();
    assert_eq!(result.get("APP_ENV").unwrap(), "development");
    assert_eq!(result.get("PORT").unwrap(), "8080");
}

#[test]
fn test_later_layer_overrides_earlier() {
    let base = make_layer(&[("APP_ENV", "development"), ("PORT", "8080")]);
    let overrides = make_layer(&[("APP_ENV", "production")]);
    let loader = EnvLoader::new().with_layer(base).with_layer(overrides);
    let result = loader.load().unwrap();
    assert_eq!(result.get("APP_ENV").unwrap(), "production");
    assert_eq!(result.get("PORT").unwrap(), "8080");
}

#[test]
fn test_load_empty_layers_returns_empty_map() {
    let loader = EnvLoader::new();
    let result = loader.load().unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_layer_count() {
    let l1 = make_layer(&[("A", "1")]);
    let l2 = make_layer(&[("B", "2")]);
    let loader = EnvLoader::new().with_layer(l1).with_layer(l2);
    assert_eq!(loader.layer_count(), 2);
}

#[test]
fn test_load_common_returns_intersection() {
    let l1 = make_layer(&[("SHARED", "v1"), ("ONLY_L1", "x")]);
    let l2 = make_layer(&[("SHARED", "v2"), ("ONLY_L2", "y")]);
    let loader = EnvLoader::new().with_layer(l1).with_layer(l2);
    let result = loader.load_common().unwrap();
    assert!(result.contains_key("SHARED"));
    assert!(!result.contains_key("ONLY_L1"));
    assert!(!result.contains_key("ONLY_L2"));
    // Value from last layer wins
    assert_eq!(result.get("SHARED").unwrap(), "v2");
}

#[test]
fn test_load_common_single_layer_returns_all() {
    let layer = make_layer(&[("A", "1"), ("B", "2")]);
    let loader = EnvLoader::new().with_layer(layer);
    let result = loader.load_common().unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_load_common_no_layers_returns_empty() {
    let loader = EnvLoader::new();
    let result = loader.load_common().unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_three_layers_merge_correctly() {
    let l1 = make_layer(&[("A", "1"), ("B", "2")]);
    let l2 = make_layer(&[("B", "3"), ("C", "4")]);
    let l3 = make_layer(&[("C", "5"), ("D", "6")]);
    let loader = EnvLoader::new()
        .with_layer(l1)
        .with_layer(l2)
        .with_layer(l3);
    let result = loader.load().unwrap();
    assert_eq!(result.get("A").unwrap(), "1");
    assert_eq!(result.get("B").unwrap(), "3");
    assert_eq!(result.get("C").unwrap(), "5");
    assert_eq!(result.get("D").unwrap(), "6");
}
