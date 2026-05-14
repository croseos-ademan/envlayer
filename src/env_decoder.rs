use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::env_encoder::EncodingFormat;

/// Decodes environment variable values from an encoded format.
#[derive(Debug, Clone)]
pub struct EnvDecoder {
    format: EncodingFormat,
    keys: Option<Vec<String>>,
}

impl EnvDecoder {
    pub fn new(format: EncodingFormat) -> Self {
        Self { format, keys: None }
    }

    pub fn with_keys(mut self, keys: Vec<String>) -> Self {
        self.keys = Some(keys);
        self
    }

    pub fn decode_value(&self, value: &str) -> Result<String, EnvLayerError> {
        match self.format {
            EncodingFormat::Base64 => base64_decode(value)
                .ok_or_else(|| EnvLayerError::InvalidValue(format!("Invalid base64: {}", value))),
            EncodingFormat::UrlEncoded => Ok(url_decode(value)),
            EncodingFormat::HexEncoded => hex_decode(value)
                .ok_or_else(|| EnvLayerError::InvalidValue(format!("Invalid hex: {}", value))),
        }
    }

    pub fn decode_map(
        &self,
        env: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = env.clone();
        for (key, value) in env.iter() {
            let should_decode = self
                .keys
                .as_ref()
                .map(|ks| ks.contains(key))
                .unwrap_or(true);
            if should_decode {
                result.insert(key.clone(), self.decode_value(value)?);
            }
        }
        Ok(result)
    }
}

fn base64_decode(input: &str) -> Option<String> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input = input.trim_end_matches('=');
    let mut bits: u32 = 0;
    let mut bit_count = 0u32;
    let mut out = Vec::new();
    for c in input.chars() {
        let val = CHARS.iter().position(|&b| b == c as u8)? as u32;
        bits = (bits << 6) | val;
        bit_count += 6;
        if bit_count >= 8 {
            bit_count -= 8;
            out.push(((bits >> bit_count) & 0xFF) as u8);
        }
    }
    String::from_utf8(out).ok()
}

fn url_decode(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '+' {
            out.push(' ');
        } else if c == '%' {
            let h1 = chars.next().and_then(|c| c.to_digit(16));
            let h2 = chars.next().and_then(|c| c.to_digit(16));
            if let (Some(h1), Some(h2)) = (h1, h2) {
                out.push(char::from_u32(h1 * 16 + h2).unwrap_or(c));
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn hex_decode(input: &str) -> Option<String> {
    if input.len() % 2 != 0 {
        return None;
    }
    let bytes: Option<Vec<u8>> = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).ok())
        .collect();
    String::from_utf8(bytes?).ok()
}
