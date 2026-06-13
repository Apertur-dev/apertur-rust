//! Server encryption key retrieval.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::ServerKey;
use std::sync::Arc;

/// Retrieve the server's encryption key for E2E encrypted uploads.
///
/// Access via [`Apertur::encryption()`](crate::Apertur::encryption).
pub struct Encryption {
    http: Arc<HttpClient>,
}

impl Encryption {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Get the server's RSA public key for E2E encryption.
    ///
    /// Use the returned PEM key with [`encrypt_image`](crate::encrypt_image) or
    /// [`Upload::image_encrypted`](crate::resources::upload::Upload::image_encrypted).
    pub fn get_server_key(&self) -> Result<ServerKey> {
        self.http
            .request("GET", "/api/v1/encryption/server-key", None)
    }
}
