use envlayer::env_sorter::{EnvSorter, SortOrder};
use envlayer::sorter_builder::SorterBuilder;
use std::collections::HashMap;

fn sample_env() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("ZEBRA".to_string(), "z".to_string());
    map.insert("ALPHA".to_string(), "a".to_string());
    map.insert("MANGO".to_string(), "m".to_string());
    map
}

#[test]
fn test_ascending_sort() {
    let sorter = EnvSorter::new(SortOrder::Ascending);
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ALPHA", "MANGO", "ZEBRA"]);
}

#[test]
fn test_descending_sort() {
    let sorter = EnvSorter::new(SortOrder::Descending);
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ZEBRA", "MANGO", "ALPHA"]);
}

#[test]
fn test_custom_sort_full_coverage() {
    let sorter = EnvSorter::new(SortOrder::Custom(vec![
        "ZEBRA".to_string(),
        "ALPHA".to_string(),
        "MANGO".to_string(),
    ]));
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ZEBRA", "ALPHA", "MANGO"]);
}

#[test]
fn test_custom_sort_partial_coverage() {
    // MANGO not in priority list — should appear after the prioritised keys, alphabetically
    let sorter = EnvSorter::new(SortOrder::Custom(vec![
        "ZEBRA".to_string(),
        "ALPHA".to_string(),
    ]));
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys[0], "ZEBRA");
    assert_eq!(keys[1], "ALPHA");
    assert_eq!(keys[2], "MANGO");
}

#[test]
fn test_sort_returns_values() {
    let sorter = EnvSorter::default();
    let pairs = sorter.sort(&sample_env());
    assert_eq!(pairs[0], ("ALPHA".to_string(), "a".to_string()));
}

#[test]
fn test_empty_map() {
    let sorter = EnvSorter::default();
    let result = sorter.sort(&HashMap::new());
    assert!(result.is_empty());
}

#[test]
fn test_builder_ascending() {
    let sorter = SorterBuilder::new().ascending().build();
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ALPHA", "MANGO", "ZEBRA"]);
}

#[test]
fn test_builder_descending() {
    let sorter = SorterBuilder::new().descending().build();
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ZEBRA", "MANGO", "ALPHA"]);
}

#[test]
fn test_builder_custom() {
    let sorter = SorterBuilder::new()
        .custom(vec!["MANGO", "ZEBRA", "ALPHA"])
        .build();
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["MANGO", "ZEBRA", "ALPHA"]);
}

#[test]
fn test_builder_default_is_ascending() {
    let sorter = SorterBuilder::new().build();
    let keys = sorter.sorted_keys(&sample_env());
    assert_eq!(keys, vec!["ALPHA", "MANGO", "ZEBRA"]);
}
