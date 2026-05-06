use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Supported export formats for resolved environment variables.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    /// Shell-compatible `export KEY=VALUE` lines
    Shell,
    /// Dotenv-style `KEY=VALUE` lines
    Dotenv,
    /// JSON object `{"KEY": "VALUE"}`
    Json,
}

/// Exports a resolved environment map into the specified format.
pub fn export(
    vars: &HashMap<String, String>,
    format: ExportFormat,
) -> Result<String, EnvLayerError> {
    if vars.is_empty() {
        return Ok(String::new());
    }

    let mut keys: Vec<&String> = vars.keys().collect();
    keys.sort();

    match format {
        ExportFormat::Shell => {
            let lines: Vec<String> = keys
                .iter()
                .map(|k| format!("export {}={}", k, shell_escape(vars[*k].as_str())))
                .collect();
            Ok(lines.join("\n"))
        }
        ExportFormat::Dotenv => {
            let lines: Vec<String> = keys
                .iter()
                .map(|k| format!("{}={}", k, vars[*k]))
                .collect();
            Ok(lines.join("\n"))
        }
        ExportFormat::Json => {
            let pairs: Vec<String> = keys
                .iter()
                .map(|k| format!("  \"{}\": \"{}\"", k, json_escape(vars[*k].as_str())))
                .collect();
            Ok(format!("{{{}}}", if pairs.is_empty() { String::new() } else { format!("\n{}\n", pairs.join(",\n")) }))
        }
    }
}

fn shell_escape(value: &str) -> String {
    if value.contains(' ') || value.contains('"') || value.contains('$') {
        format!("'{}'", value.replace('\'', "'\\''" ))
    } else {
        value.to_string()
    }
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
