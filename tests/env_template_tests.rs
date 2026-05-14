use envlayer::env_template::EnvTemplate;
use std::collections::HashMap;

fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
test_render_simple_placeholder() {
    let tmpl = EnvTemplate::new("Hello, ${NAME}!");
    let v = vars(&[("NAME", "world")]);
    assert_eq!(tmpl.render(&v).unwrap(), "Hello, world!");
}

#[test]
fn test_render_multiple_placeholders() {
    let tmpl = EnvTemplate::new("${GREETING}, ${NAME}!");
    let v = vars(&[("GREETING", "Hi"), ("NAME", "Alice")]);
    assert_eq!(tmpl.render(&v).unwrap(), "Hi, Alice!");
}

#[test]
fn test_render_missing_key_returns_error() {
    let tmpl = EnvTemplate::new("${MISSING}");
    let v = vars(&[]);
    assert!(tmpl.render(&v).is_err());
}

#[test]
fn test_render_no_placeholders() {
    let tmpl = EnvTemplate::new("static value");
    let v = vars(&[]);
    assert_eq!(tmpl.render(&v).unwrap(), "static value");
}

#[test]
fn test_render_partial_leaves_missing() {
    let tmpl = EnvTemplate::new("${KNOWN}-${UNKNOWN}");
    let v = vars(&[("KNOWN", "hello")]);
    assert_eq!(tmpl.render_partial(&v), "hello-${UNKNOWN}");
}

#[test]
fn test_render_partial_all_resolved() {
    let tmpl = EnvTemplate::new("${A}/${B}");
    let v = vars(&[("A", "foo"), ("B", "bar")]);
    assert_eq!(tmpl.render_partial(&v), "foo/bar");
}

#[test]
fn test_raw_returns_original() {
    let raw = "prefix_${VAR}_suffix";
    let tmpl = EnvTemplate::new(raw);
    assert_eq!(tmpl.raw(), raw);
}

#[test]
fn test_render_adjacent_placeholders() {
    let tmpl = EnvTemplate::new("${A}${B}");
    let v = vars(&[("A", "foo"), ("B", "bar")]);
    assert_eq!(tmpl.render(&v).unwrap(), "foobar");
}
