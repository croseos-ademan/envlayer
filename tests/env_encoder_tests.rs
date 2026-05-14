use std::collections::HashMap;
use envlayer::env_encoder::{EncodingFormat, EnvEncoder};
use envlayer::env_decoder::EnvDecoder;

fn make_map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_base64_encode_decode_roundtrip() {
    let encoder = EnvEncoder::new(EncodingFormat::Base64);
    let decoder = EnvDecoder::new(EncodingFormat::Base64);
    let original = "hello world";
    let encoded = encoder.encode_value(original).unwrap();
    let decoded = decoder.decode_value(&encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_hex_encode_decode_roundtrip() {
    let encoder = EnvEncoder::new(EncodingFormat::HexEncoded);
    let decoder = EnvDecoder::new(EncodingFormat::HexEncoded);
    let original = "secret_value_123";
    let encoded = encoder.encode_value(original).unwrap();
    assert!(encoded.chars().all(|c| c.is_ascii_hexdigit()));
    let decoded = decoder.decode_value(&encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_url_encode_decode_roundtrip() {
    let encoder = EnvEncoder::new(EncodingFormat::UrlEncoded);
    let decoder = EnvDecoder::new(EncodingFormat::UrlEncoded);
    let original = "hello world&foo=bar";
    let encoded = encoder.encode_value(original).unwrap();
    let decoded = decoder.decode_value(&encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_encode_map_all_keys() {
    let encoder = EnvEncoder::new(EncodingFormat::HexEncoded);
    let env = make_map(&[("KEY1", "abc"), ("KEY2", "xyz")]);
    let encoded = encoder.encode_map(&env).unwrap();
    assert_eq!(encoded["KEY1"], "616263");
    assert_eq!(encoded["KEY2"], "78797a");
}

#[test]
fn test_encode_map_selective_keys() {
    let encoder = EnvEncoder::new(EncodingFormat::HexEncoded)
        .with_keys(vec!["SECRET".to_string()]);
    let env = make_map(&[("SECRET", "abc"), ("PLAIN", "xyz")]);
    let encoded = encoder.encode_map(&env).unwrap();
    assert_eq!(encoded["SECRET"], "616263");
    assert_eq!(encoded["PLAIN"], "xyz");
}

#[test]
fn test_decode_map_selective_keys() {
    let decoder = EnvDecoder::new(EncodingFormat::HexEncoded)
        .with_keys(vec!["SECRET".to_string()]);
    let env = make_map(&[("SECRET", "616263"), ("PLAIN", "xyz")]);
    let decoded = decoder.decode_map(&env).unwrap();
    assert_eq!(decoded["SECRET"], "abc");
    assert_eq!(decoded["PLAIN"], "xyz");
}

#[test]
fn test_invalid_hex_returns_error() {
    let decoder = EnvDecoder::new(EncodingFormat::HexEncoded);
    let result = decoder.decode_value("xyz");
    assert!(result.is_err());
}

#[test]
fn test_url_encode_special_chars() {
    let encoder = EnvEncoder::new(EncodingFormat::UrlEncoded);
    let encoded = encoder.encode_value("a b+c").unwrap();
    assert!(encoded.contains("%2B") || encoded.contains("+"));
    assert!(!encoded.contains(' '));
}

#[test]
fn test_base64_empty_string() {
    let encoder = EnvEncoder::new(EncodingFormat::Base64);
    let decoder = EnvDecoder::new(EncodingFormat::Base64);
    let encoded = encoder.encode_value("").unwrap();
    let decoded = decoder.decode_value(&encoded).unwrap();
    assert_eq!(decoded, "");
}
