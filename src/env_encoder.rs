use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Encoding format for environment variable serialization.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodingFormat {
    Base64,
    UrlEncoded,
    HexEncoded,
}

/// Encodes environment variable values into a target format.
#[derive(Debug, Clone)]
pub struct EnvEncoder {
    format: EncodingFormat,
    keys: Option<Vec<String>>,
}

impl EnvEncoder {
    pub fn new(format: EncodingFormat) -> Self {
        Self { format, keys: None }
    }

    /// Restrict encoding to specific keys only.
    pub fn with_keys(mut self, keys: Vec<String>) -> Self {
        self.keys = Some(keys);
        self
    }

    /// Encode a single value using the configured format.
    pub fn encode_value(&self, value: &str) -> Result<String, EnvLayerError> {
        match self.format {
            EncodingFormat::Base64 => Ok(base64_encode(value)),
            EncodingFormat::UrlEncoded => Ok(url_encode(value)),
            EncodingFormat::HexEncoded => Ok(hex_encode(value)),
        }
    }

    /// Encode all (or selected) values in the map.
    pub fn encode_map(
        &self,
        env: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = env.clone();
        for (key, value) in env.iter() {
            let should_encode = self
                .keys
                .as_ref()
                .map(|ks| ks.contains(key))
                .unwrap_or(true);
            if should_encode {
                result.insert(key.clone(), self.encode_value(value)?);
            }
        }
        Ok(result)
    }
}

fn base64_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        out.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        out.push(if chunk.len() > 1 { CHARS[((n >> 6) & 0x3F) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[(n & 0x3F) as usize] as char } else { '=' });
    }
    out
}

fn url_encode(input: &str) -> String {
    input
        .chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => vec![c],
            ' ' => vec!['+'],
            _ => format!("%{:02X}", c as u32).chars().collect(),
        })
        .collect()
}

fn hex_encode(input: &str) -> String {
    input.bytes().map(|b| format!("{:02x}", b)).collect()
}
