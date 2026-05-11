use envlayer::env_context::{ContextKind, EnvContext};

#[test]
fn test_context_kind_as_str() {
    assert_eq!(ContextKind::Local.as_str(), "local");
    assert_eq!(ContextKind::CI.as_str(), "ci");
    assert_eq!(ContextKind::Docker.as_str(), "docker");
    assert_eq!(ContextKind::Custom("staging".to_string()).as_str(), "staging");
}

#[test]
fn test_context_kind_from_str() {
    assert_eq!(ContextKind::from_str("local"), ContextKind::Local);
    assert_eq!(ContextKind::from_str("CI"), ContextKind::CI);
    assert_eq!(ContextKind::from_str("docker"), ContextKind::Docker);
    assert_eq!(
        ContextKind::from_str("staging"),
        ContextKind::Custom("staging".to_string())
    );
}

#[test]
fn test_env_context_new() {
    let ctx = EnvContext::new(ContextKind::Local);
    assert!(ctx.is_local());
    assert!(!ctx.is_ci());
    assert!(ctx.metadata.is_empty());
}

#[test]
fn test_env_context_with_metadata() {
    let ctx = EnvContext::new(ContextKind::CI)
        .with_metadata("runner", "ubuntu-latest")
        .with_metadata("branch", "main");
    assert_eq!(ctx.metadata.get("runner"), Some(&"ubuntu-latest".to_string()));
    assert_eq!(ctx.metadata.get("branch"), Some(&"main".to_string()));
}

#[test]
fn test_env_context_is_ci() {
    let ctx = EnvContext::new(ContextKind::CI);
    assert!(ctx.is_ci());
    assert!(!ctx.is_local());
}

#[test]
fn test_env_context_is_local() {
    let ctx = EnvContext::new(ContextKind::Local);
    assert!(ctx.is_local());
    assert!(!ctx.is_ci());
}

#[test]
fn test_env_context_custom_kind() {
    let ctx = EnvContext::new(ContextKind::Custom("staging".to_string()));
    assert!(!ctx.is_local());
    assert!(!ctx.is_ci());
    assert_eq!(ctx.kind.as_str(), "staging");
}
