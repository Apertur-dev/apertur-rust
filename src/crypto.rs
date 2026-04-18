//! Image encryption utilities.
//!
//! Provides AES-256-GCM encryption with RSA-OAEP key wrapping for end-to-end
//! encrypted image uploads.

use crate::error::{AperturError, Result};
use crate::types::EncryptedPayload;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit};
use base64::Engine;
use rand::RngCore;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;

/// Encrypt image data for end-to-end encrypted upload.
///
/// This function:
/// 1. Generates a random 32-byte AES-256 key and 12-byte IV.
/// 2. Encrypts the image data with AES-256-GCM (the auth tag is appended to the ciphertext).
/// 3. Wraps the AES key with RSA-OAEP (SHA-256) using the provided PEM public key.
/// 4. Returns all components base64-encoded in an [`EncryptedPayload`].
///
/// # Arguments
///
/// * `image_data` - The raw image bytes to encrypt.
/// * `public_key_pem` - The server's RSA public key in PEM format.
///
/// # Errors
///
/// Returns [`AperturError::Encryption`] if key parsing, encryption, or key wrapping fails.
pub fn encrypt_image(image_data: &[u8], public_key_pem: &str) -> Result<EncryptedPayload> {
    let engine = base64::engine::general_purpose::STANDARD;

    // Generate random AES-256 key and 12-byte IV
    let mut aes_key = [0u8; 32];
    let mut iv = [0u8; 12];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut aes_key);
    rng.fill_bytes(&mut iv);

    // Encrypt image with AES-256-GCM
    let cipher = Aes256Gcm::new_from_slice(&aes_key)
        .map_err(|e| AperturError::Encryption(format!("Failed to create AES cipher: {}", e)))?;

    let nonce = GenericArray::from_slice(&iv);
    let encrypted_data = cipher
        .encrypt(nonce, image_data)
        .map_err(|e| AperturError::Encryption(format!("AES-GCM encryption failed: {}", e)))?;

    // Parse RSA public key and wrap AES key with RSA-OAEP
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| AperturError::Encryption(format!("Failed to parse RSA public key: {}", e)))?;

    let padding = Oaep::new::<Sha256>();
    let wrapped_key = public_key
        .encrypt(&mut rng, padding, &aes_key)
        .map_err(|e| AperturError::Encryption(format!("RSA-OAEP encryption failed: {}", e)))?;

    Ok(EncryptedPayload {
        encrypted_key: engine.encode(&wrapped_key),
        iv: engine.encode(iv),
        encrypted_data: engine.encode(&encrypted_data),
        algorithm: "RSA-OAEP+AES-256-GCM".to_string(),
    })
}
