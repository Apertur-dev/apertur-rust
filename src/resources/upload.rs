//! Image upload operations.

use crate::crypto::encrypt_image;
use crate::error::{AperturError, Result};
use crate::http::HttpClient;
use crate::types::{UploadFile, UploadOptions, UploadResult};
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

/// Upload images to a session.
///
/// Access via [`Apertur::upload()`](crate::Apertur::upload).
pub struct Upload {
    http: Arc<HttpClient>,
}

impl Upload {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Upload an image to a session.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The session UUID.
    /// * `file` - The image file as a path or in-memory bytes.
    /// * `options` - Upload options (filename override, MIME type, source, password).
    pub fn image(
        &self,
        uuid: &str,
        file: UploadFile,
        options: &UploadOptions,
    ) -> Result<UploadResult> {
        let (data, default_filename) = match file {
            UploadFile::Path(ref path) => {
                let bytes = std::fs::read(path).map_err(AperturError::Io)?;
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("image.jpg")
                    .to_string();
                (bytes, name)
            }
            UploadFile::Bytes(data, ref name) => (data, name.clone()),
        };

        let filename = options.filename.as_deref().unwrap_or(&default_filename);
        let mime_type = options
            .mime_type
            .as_deref()
            .unwrap_or("image/jpeg");

        let file_part = reqwest::blocking::multipart::Part::bytes(data)
            .file_name(filename.to_string())
            .mime_str(mime_type)
            .map_err(|e| AperturError::Encryption(format!("Invalid MIME type: {}", e)))?;

        let mut form = reqwest::blocking::multipart::Form::new().part("file", file_part);

        if let Some(ref source) = options.source {
            form = form.text("source", source.clone());
        }

        let mut extra_headers = HeaderMap::new();
        if let Some(ref password) = options.password {
            extra_headers.insert(
                "x-session-password",
                HeaderValue::from_str(password)
                    .map_err(|e| AperturError::Encryption(format!("Invalid password header: {}", e)))?,
            );
        }

        let headers = if extra_headers.is_empty() {
            None
        } else {
            Some(extra_headers)
        };

        self.http.request_multipart(
            &format!("/api/v1/upload/{}/images", uuid),
            form,
            headers,
        )
    }

    /// Upload an encrypted image to a session.
    ///
    /// The image is encrypted client-side using AES-256-GCM with an RSA-OAEP
    /// wrapped key before being sent to the server.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The session UUID.
    /// * `data` - The raw image bytes.
    /// * `public_key` - The server's RSA public key in PEM format (from [`Encryption::get_server_key`](crate::resources::encryption::Encryption::get_server_key)).
    /// * `options` - Upload options.
    pub fn image_encrypted(
        &self,
        uuid: &str,
        data: &[u8],
        public_key: &str,
        options: &UploadOptions,
    ) -> Result<UploadResult> {
        let encrypted = encrypt_image(data, public_key)?;

        let filename = options.filename.as_deref().unwrap_or("image.jpg");
        let mime_type = options.mime_type.as_deref().unwrap_or("image/jpeg");
        let source = options.source.as_deref().unwrap_or("sdk");

        let payload = serde_json::json!({
            "encryptedKey": encrypted.encrypted_key,
            "iv": encrypted.iv,
            "encryptedData": encrypted.encrypted_data,
            "algorithm": encrypted.algorithm,
            "filename": filename,
            "mimeType": mime_type,
            "source": source,
        });

        let mut extra_headers = HeaderMap::new();
        extra_headers.insert(
            "X-Aptr-Encrypted",
            HeaderValue::from_static("default"),
        );
        if let Some(ref password) = options.password {
            extra_headers.insert(
                "x-session-password",
                HeaderValue::from_str(password)
                    .map_err(|e| AperturError::Encryption(format!("Invalid password header: {}", e)))?,
            );
        }

        self.http.request_json_with_headers(
            &format!("/api/v1/upload/{}/images", uuid),
            &serde_json::to_string(&payload)?,
            extra_headers,
        )
    }
}
