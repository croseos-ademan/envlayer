use envlayer::profile::Profile;
use envlayer::error::EnvLayerError;

#[test]
fn test_profile_creation() {
    let p = Profile::new("local");
    assert_eq!(p.name, "local");
    assert!(p.description.is_none());
    assert!(p.tags.is_empty());
    assert!(p.vars.is_empty());
}

#[test]
fn test_profile_set_and_get() {
    let mut p = Profile::new("ci");
    p.set("API_URL", "https://ci.example.com");
    assert_eq!(p.get("API_URL"), Some(&"https://ci.example.com".to_string()));
    assert_eq!(p.get("MISSING"), None);
}

#[test]
fn test_profile_with_description_and_tag() {
    let p = Profile::new("staging")
        .with_description("Staging environment")
        .with_tag("cloud")
        .with_tag("aws");
    assert_eq!(p.description.as_deref(), Some("Staging environment"));
    assert_eq!(p.tags, vec!["cloud", "aws"]);
}

#[test]
fn test_profile_merge_from_does_not_overwrite() {
    let mut base = Profile::new("base");
    base.set("KEY", "base_value");

    let mut other = Profile::new("other");
    other.set("KEY", "other_value");
    other.set("EXTRA", "extra_value");

    base.merge_from(&other);
    assert_eq!(base.get("KEY").map(String::as_str), Some("base_value"));
    assert_eq!(base.get("EXTRA").map(String::as_str), Some("extra_value"));
}

#[test]
fn test_validate_name_empty() {
    let result = Profile::validate_name("");
    assert!(matches!(result, Err(EnvLayerError::InvalidInput(_))));
}

#[test]
fn test_validate_name_whitespace() {
    let result = Profile::validate_name("my profile");
    assert!(matches!(result, Err(EnvLayerError::InvalidInput(_))));
}

#[test]
fn test_validate_name_valid() {
    assert!(Profile::validate_name("local").is_ok());
    assert!(Profile::validate_name("ci-prod").is_ok());
}
