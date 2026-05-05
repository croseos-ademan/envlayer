use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

use envlayer::source::Source;

#[test]
fn test_inline_source_loads_pairs() {
    let src = Source::inline([("FOO", "bar"), ("BAZ", "qux")]);
    let vars = src.load().expect("inline load should succeed");
    assert_eq!(vars.get("FOO").map(String::as_str), Some("bar"));
    assert_eq!(vars.get("BAZ").map(String::as_str), Some("qux"));
}

#[test]
fn test_dotenv_file_parses_correctly() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "# a comment").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "APP_ENV=production").unwrap();
    writeln!(file, "DATABASE_URL=\"postgres://localhost/mydb\"").unwrap();
    writeln!(file, "PORT=8080").unwrap();

    let src = Source::DotenvFile(file.path().to_string_lossy().into_owned());
    let vars = src.load().expect("dotenv load should succeed");

    assert_eq!(vars.get("APP_ENV").map(String::as_str), Some("production"));
    assert_eq!(vars.get("DATABASE_URL").map(String::as_str), Some("postgres://localhost/mydb"));
    assert_eq!(vars.get("PORT").map(String::as_str), Some("8080"));
    assert!(!vars.contains_key("# a comment"));
}

#[test]
fn test_dotenv_file_not_found_returns_error() {
    let src = Source::DotenvFile("/nonexistent/path/.env".to_string());
    let result = src.load();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Failed to read"));
}

#[test]
fn test_dotenv_invalid_line_returns_parse_error() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "VALID=ok").unwrap();
    writeln!(file, "INVALID_LINE_NO_EQUALS").unwrap();

    let src = Source::DotenvFile(file.path().to_string_lossy().into_owned());
    let result = src.load();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("KEY=VALUE"));
}

#[test]
fn test_system_env_source_contains_path() {
    let src = Source::SystemEnv;
    let vars = src.load().expect("system env load should succeed");
    // PATH is virtually always set in any environment
    assert!(vars.contains_key("PATH"), "expected PATH in system env");
}

#[test]
fn test_inline_empty_source() {
    let src = Source::inline(std::iter::empty::<(String, String)>());
    let vars = src.load().expect("empty inline load should succeed");
    assert!(vars.is_empty());
}
