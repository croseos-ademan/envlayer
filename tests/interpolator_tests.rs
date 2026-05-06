use envlayer::interpolator::Interpolator;
use envlayer::error::EnvLayerError;
use std::collections::HashMap;

fn ctx(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
test_no_variables() {
    let interp = Interpolator::new(false);
    let result = interp.interpolate("hello world", &ctx(&[])).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_curly_brace_syntax() {
    let interp = Interpolator::new(false);
    let context = ctx(&[("HOME", "/home/user")]);
    let result = interp.interpolate("path=${HOME}/bin", &context).unwrap();
    assert_eq!(result, "path=/home/user/bin");
}

#[test]
fn test_bare_dollar_syntax() {
    let interp = Interpolator::new(false);
    let context = ctx(&[("USER", "alice")]);
    let result = interp.interpolate("Hello $USER!", &context).unwrap();
    assert_eq!(result, "Hello alice!");
}

#[test]
fn test_multiple_variables() {
    let interp = Interpolator::new(false);
    let context = ctx(&[("FIRST", "foo"), ("SECOND", "bar")]);
    let result = interp
        .interpolate("${FIRST}-${SECOND}", &context)
        .unwrap();
    assert_eq!(result, "foo-bar");
}

#[test]
fn test_missing_variable_lenient() {
    let interp = Interpolator::new(false);
    let result = interp.interpolate("${MISSING}", &ctx(&[])).unwrap();
    assert_eq!(result, "${MISSING}");
}

#[test]
fn test_missing_variable_strict() {
    let interp = Interpolator::new(true);
    let err = interp.interpolate("${MISSING}", &ctx(&[])).unwrap_err();
    assert_eq!(err, EnvLayerError::MissingVariable("MISSING".to_string()));
}

#[test]
fn test_lone_dollar_preserved() {
    let interp = Interpolator::new(false);
    let result = interp.interpolate("cost is $5", &ctx(&[])).unwrap();
    assert_eq!(result, "cost is $5");
}

#[test]
fn test_nested_value_not_re_interpolated() {
    let interp = Interpolator::new(false);
    let context = ctx(&[("A", "${B}"), ("B", "deep")]);
    // Interpolator does NOT recursively expand; A resolves to literal "${B}"
    let result = interp.interpolate("${A}", &context).unwrap();
    assert_eq!(result, "${B}");
}

#[test]
fn test_empty_value() {
    let interp = Interpolator::new(false);
    let context = ctx(&[("EMPTY", "")]);
    let result = interp.interpolate("prefix_${EMPTY}_suffix", &context).unwrap();
    assert_eq!(result, "prefix__suffix");
}
