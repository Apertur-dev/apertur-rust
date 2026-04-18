//! Webhook signature verification utilities.
//!
//! These free functions verify HMAC-based signatures used by Apertur webhooks.
//! All comparisons use constant-time equality to prevent timing attacks.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify an image delivery webhook signature.
///
/// The signature header (`X-Apertur-Signature`) has the form `sha256=<hex>`.
/// The expected value is `HMAC-SHA256(body, secret)` encoded as lowercase hex.
///
/// # Arguments
///
/// * `body` - The raw request body as a string.
/// * `signature` - The value of the `X-Apertur-Signature` header.
/// * `secret` - The webhook signing secret.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
pub fn verify_webhook_signature(body: &str, signature: &str, secret: &str) -> bool {
    let sig = signature.strip_prefix("sha256=").unwrap_or(signature);

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(body.as_bytes());
    let expected = hex_encode(&mac.finalize().into_bytes());

    constant_time_eq(expected.as_bytes(), sig.as_bytes())
}

/// Verify an event webhook signature (HMAC SHA256 method).
///
/// The signed payload is `{timestamp}.{body}`. The signature header
/// (`X-Apertur-Signature`) has the form `sha256=<hex>`.
///
/// # Arguments
///
/// * `body` - The raw request body as a string.
/// * `timestamp` - The value of the `X-Apertur-Timestamp` header (unix seconds).
/// * `signature` - The value of the `X-Apertur-Signature` header.
/// * `secret` - The webhook signing secret.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
pub fn verify_event_signature(
    body: &str,
    timestamp: &str,
    signature: &str,
    secret: &str,
) -> bool {
    let sig = signature.strip_prefix("sha256=").unwrap_or(signature);

    let signature_base = format!("{}.{}", timestamp, body);
    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(signature_base.as_bytes());
    let expected = hex_encode(&mac.finalize().into_bytes());

    constant_time_eq(expected.as_bytes(), sig.as_bytes())
}

/// Verify an event webhook signature (Svix method).
///
/// The signed payload is `{svix_id}.{timestamp}.{body}`. The secret is decoded
/// from hex before being used as the HMAC key. The signature header
/// (`svix-signature`) has the form `v1,<base64>`.
///
/// # Arguments
///
/// * `body` - The raw request body as a string.
/// * `svix_id` - The value of the `svix-id` header.
/// * `timestamp` - The value of the `svix-timestamp` header.
/// * `signature` - The value of the `svix-signature` header.
/// * `secret` - The webhook signing secret (hex-encoded).
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
pub fn verify_svix_signature(
    body: &str,
    svix_id: &str,
    timestamp: &str,
    signature: &str,
    secret: &str,
) -> bool {
    use base64::Engine;
    let engine = base64::engine::general_purpose::STANDARD;

    let sig = signature.strip_prefix("v1,").unwrap_or(signature);

    let secret_bytes = match hex_decode(secret) {
        Some(b) => b,
        None => return false,
    };

    let signature_base = format!("{}.{}.{}", svix_id, timestamp, body);
    let mut mac = match HmacSha256::new_from_slice(&secret_bytes) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(signature_base.as_bytes());
    let expected_bytes = mac.finalize().into_bytes();
    let expected_b64 = engine.encode(expected_bytes);

    let sig_bytes = match engine.decode(sig) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let expected_decoded = match engine.decode(&expected_b64) {
        Ok(b) => b,
        Err(_) => return false,
    };

    constant_time_eq(&expected_decoded, &sig_bytes)
}

/// Constant-time byte-slice comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Encode bytes as lowercase hex.
fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Decode a hex string to bytes.
fn hex_decode(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).ok()?;
        bytes.push(byte);
    }
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_webhook_signature() {
        let secret = "test_secret";
        let body = r#"{"event":"test"}"#;

        // Compute expected signature
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        let expected_hex = hex_encode(&mac.finalize().into_bytes());

        let signature = format!("sha256={}", expected_hex);
        assert!(verify_webhook_signature(body, &signature, secret));
        assert!(!verify_webhook_signature(body, "sha256=invalid", secret));
    }

    #[test]
    fn test_verify_event_signature() {
        let secret = "event_secret";
        let body = r#"{"type":"upload.completed"}"#;
        let timestamp = "1700000000";

        let signature_base = format!("{}.{}", timestamp, body);
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signature_base.as_bytes());
        let expected_hex = hex_encode(&mac.finalize().into_bytes());

        let signature = format!("sha256={}", expected_hex);
        assert!(verify_event_signature(body, timestamp, &signature, secret));
        assert!(!verify_event_signature(body, "9999999999", &signature, secret));
    }

    #[test]
    fn test_verify_svix_signature() {
        use base64::Engine;
        let engine = base64::engine::general_purpose::STANDARD;

        let secret_bytes = b"svix_secret_key_1234567890abcdef";
        let secret_hex = hex_encode(secret_bytes);
        let body = r#"{"type":"upload.completed"}"#;
        let svix_id = "msg_abc123";
        let timestamp = "1700000000";

        let signature_base = format!("{}.{}.{}", svix_id, timestamp, body);
        let mut mac = HmacSha256::new_from_slice(secret_bytes).unwrap();
        mac.update(signature_base.as_bytes());
        let sig_b64 = engine.encode(mac.finalize().into_bytes());

        let signature = format!("v1,{}", sig_b64);
        assert!(verify_svix_signature(body, svix_id, timestamp, &signature, &secret_hex));
        assert!(!verify_svix_signature(body, svix_id, "wrong", &signature, &secret_hex));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
    }
}
