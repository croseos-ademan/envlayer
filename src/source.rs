//! Environment variable sources: dotenv files, system env, and inline maps.

use std::collections::HashMap;
use std::path::Path;
use std::fs;

use crate::error::{EnvLayerError, Result};

/// A source of environment variable key-value pairs.
#[derive(Debug, Clone)]
pub enum Source {
    /// Load variables from a `.env`-style file.
    DotenvFile(String),
    /// Load variables from the current process environment.
    SystemEnv,
    /// Inline key-value pairs provided directly.
    Inline(HashMap<String, String>),
}

impl Source {
    /// Resolve this source into a flat map of key-value pairs.
    pub fn load(&self) -> Result<HashMap<String, String>> {
        match self {
            Source::DotenvFile(path) => load_dotenv(path),
            Source::SystemEnv => Ok(std::env::vars().collect()),
            Source::Inline(map) => Ok(map.clone()),
        }
    }

    /// Convenience constructor for an inline source.
    pub fn inline<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        Source::Inline(pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}

/// Parse a `.env`-style file into a key-value map.
/// Supports `KEY=VALUE` lines; ignores comments (`#`) and blank lines.
fn load_dotenv<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|e| {
        EnvLayerError::Io(format!("Failed to read '{}': {}", path.display(), e))
    })?;

    let mut map = HashMap::new();
    for (line_no, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed.split_once('=').ok_or_else(|| {
            EnvLayerError::Parse(format!(
                "{}:{}: expected KEY=VALUE, got: {}",
                path.display(),
                line_no + 1,
                trimmed
            ))
        })?;
        map.insert(key.trim().to_string(), value.trim().trim_matches('"').to_string());
    }
    Ok(map)
}
