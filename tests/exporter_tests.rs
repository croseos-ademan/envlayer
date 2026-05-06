use std::collections::HashMap;
use envlayer::exporter::{export, ExportFormat};

fn sample_vars() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("APP_ENV".to_string(), "production".to_string());
    map.insert("DB_HOST".to_string(), "localhost".to_string());
    map.insert("DB_PORT".to_string(), "5432".to_string());
    map
}

#[test]
fn test_export_dotenv_format() {
    let vars = sample_vars();
    let output = export(&vars, ExportFormat::Dotenv).unwrap();
    assert!(output.contains("APP_ENV=production"));
    assert!(output.contains("DB_HOST=localhost"));
    assert!(output.contains("DB_PORT=5432"));
}

#[test]
fn test_export_shell_format() {
    let vars = sample_vars();
    let output = export(&vars, ExportFormat::Shell).unwrap();
    assert!(output.contains("export APP_ENV=production"));
    assert!(output.contains("export DB_HOST=localhost"));
}

#[test]
fn test_export_shell_escapes_spaces() {
    let mut vars = HashMap::new();
    vars.insert("GREETING".to_string(), "hello world".to_string());
    let output = export(&vars, ExportFormat::Shell).unwrap();
    assert!(output.contains("export GREETING='hello world'"));
}

#[test]
fn test_export_json_format() {
    let vars = sample_vars();
    let output = export(&vars, ExportFormat::Json).unwrap();
    assert!(output.starts_with('{'));
    assert!(output.ends_with('}'));
    assert!(output.contains("\"APP_ENV\": \"production\""));
    assert!(output.contains("\"DB_HOST\": \"localhost\""));
}

#[test]
fn test_export_json_escapes_special_chars() {
    let mut vars = HashMap::new();
    vars.insert("MSG".to_string(), "line1\nline2".to_string());
    let output = export(&vars, ExportFormat::Json).unwrap();
    assert!(output.contains("\\n"));
}

#[test]
fn test_export_empty_map_returns_empty_string() {
    let vars = HashMap::new();
    let shell = export(&vars, ExportFormat::Shell).unwrap();
    let dotenv = export(&vars, ExportFormat::Dotenv).unwrap();
    let json = export(&vars, ExportFormat::Json).unwrap();
    assert!(shell.is_empty());
    assert!(dotenv.is_empty());
    assert!(json.is_empty());
}

#[test]
fn test_export_dotenv_sorted_output() {
    let vars = sample_vars();
    let output = export(&vars, ExportFormat::Dotenv).unwrap();
    let lines: Vec<&str> = output.lines().collect();
    let keys: Vec<&str> = lines.iter().map(|l| l.split('=').next().unwrap()).collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
}
