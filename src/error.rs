//! Error types for the Apertur SDK.

/// All possible errors returned by the Apertur SDK.
#[derive(Debug, thiserror::Error)]
pub enum AperturError {
    /// Authentication failed (HTTP 401).
    #[error("Authentication failed: {message}")]
    Authentication {
        /// HTTP status code.
        status_code: u16,
        /// Machine-readable error code from the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },

    /// Resource not found (HTTP 404).
    #[error("Not found: {message}")]
    NotFound {
        /// HTTP status code.
        status_code: u16,
        /// Machine-readable error code from the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },

    /// Rate limit exceeded (HTTP 429).
    #[error("Rate limited: {message}")]
    RateLimit {
        /// HTTP status code.
        status_code: u16,
        /// Machine-readable error code from the API.
        code: String,
        /// Human-readable error message.
        message: String,
        /// Seconds to wait before retrying, if provided by the server.
        retry_after: Option<u64>,
    },

    /// Validation error (HTTP 400).
    #[error("Validation error: {message}")]
    Validation {
        /// HTTP status code.
        status_code: u16,
        /// Machine-readable error code from the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },

    /// Generic API error for any other HTTP error status.
    #[error("API error ({status_code}): {message}")]
    Api {
        /// HTTP status code.
        status_code: u16,
        /// Machine-readable error code from the API.
        code: String,
        /// Human-readable error message.
        message: String,
    },

    /// HTTP transport error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error (e.g. reading a file for upload).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Encryption or decryption error.
    #[error("Encryption error: {0}")]
    Encryption(String),
}

/// A specialized `Result` type for Apertur SDK operations.
pub type Result<T> = std::result::Result<T, AperturError>;
