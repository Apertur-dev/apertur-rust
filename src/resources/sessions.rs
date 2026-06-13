//! Upload session management.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{
    DeliveryStatusOptions, DeliveryStatusResponse, ListParams, QrOptions, Session, SessionDetail,
    SessionPage, SessionRow, SessionCreateOptions, SessionUpdateOptions,
};
use std::sync::Arc;

/// Manage upload sessions.
///
/// Access via [`Apertur::sessions()`](crate::Apertur::sessions).
pub struct Sessions {
    http: Arc<HttpClient>,
}

impl Sessions {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Create a new upload session.
    ///
    /// Returns the session UUID, upload URL, QR code URL, and attached destinations.
    pub fn create(&self, options: &SessionCreateOptions) -> Result<Session> {
        let body = serde_json::to_value(options)?;
        self.http.request("POST", "/api/v1/upload-sessions", Some(&body))
    }

    /// Get detailed information about a session.
    pub fn get(&self, uuid: &str) -> Result<SessionDetail> {
        self.http
            .request("GET", &format!("/api/v1/upload/{}/session", uuid), None)
    }

    /// Update a session's configuration.
    pub fn update(&self, uuid: &str, options: &SessionUpdateOptions) -> Result<SessionDetail> {
        let body = serde_json::to_value(options)?;
        self.http.request(
            "PATCH",
            &format!("/api/v1/upload-sessions/{}", uuid),
            Some(&body),
        )
    }

    /// List sessions with pagination.
    pub fn list(&self, params: &ListParams) -> Result<SessionPage> {
        let mut qs = Vec::new();
        if let Some(page) = params.page {
            qs.push(format!("page={}", page));
        }
        if let Some(page_size) = params.page_size {
            qs.push(format!("pageSize={}", page_size));
        }
        let suffix = if qs.is_empty() {
            String::new()
        } else {
            format!("?{}", qs.join("&"))
        };
        self.http
            .request("GET", &format!("/api/v1/sessions{}", suffix), None)
    }

    /// Get recent sessions.
    pub fn recent(&self, params: &ListParams) -> Result<Vec<SessionRow>> {
        let mut qs = Vec::new();
        if let Some(limit) = params.limit {
            qs.push(format!("limit={}", limit));
        }
        let suffix = if qs.is_empty() {
            String::new()
        } else {
            format!("?{}", qs.join("&"))
        };
        self.http
            .request("GET", &format!("/api/v1/sessions/recent{}", suffix), None)
    }

    /// Download the QR code for a session as raw bytes.
    ///
    /// The returned bytes are in the format specified by [`QrOptions::format`]
    /// (defaults to PNG).
    pub fn qr(&self, uuid: &str, options: &QrOptions) -> Result<Vec<u8>> {
        let mut qs = Vec::new();
        if let Some(ref format) = options.format {
            qs.push(format!("format={}", format));
        }
        if let Some(size) = options.size {
            qs.push(format!("size={}", size));
        }
        if let Some(ref style) = options.style {
            qs.push(format!("style={}", style));
        }
        if let Some(ref fg) = options.fg {
            qs.push(format!("fg={}", fg));
        }
        if let Some(ref bg) = options.bg {
            qs.push(format!("bg={}", bg));
        }
        if let Some(border_size) = options.border_size {
            qs.push(format!("borderSize={}", border_size));
        }
        if let Some(ref border_color) = options.border_color {
            qs.push(format!("borderColor={}", border_color));
        }
        let suffix = if qs.is_empty() {
            String::new()
        } else {
            format!("?{}", qs.join("&"))
        };
        self.http.request_raw(
            "GET",
            &format!("/api/v1/upload-sessions/{}/qr{}", uuid, suffix),
        )
    }

    /// Verify a session password.
    pub fn verify_password(&self, uuid: &str, password: &str) -> Result<serde_json::Value> {
        let body = serde_json::json!({ "password": password });
        self.http.request(
            "POST",
            &format!("/api/v1/upload/{}/verify-password", uuid),
            Some(&body),
        )
    }

    /// Get the delivery status of all images in a session.
    ///
    /// Returns the overall session status, the per-file delivery states, and
    /// the timestamp of the most recent change.
    ///
    /// When [`DeliveryStatusOptions::poll_from`] is set to an ISO 8601
    /// timestamp, the server long-polls for up to 5 minutes waiting for a
    /// change past that cursor. Because the blocking `reqwest` client used
    /// here does not support per-request timeouts, callers that want to
    /// long-poll should build their [`Apertur`](crate::Apertur) client with a
    /// request timeout of at least 6 minutes
    /// (`Duration::from_secs(360)`) before calling this endpoint, so the
    /// server releases the response first under the happy path.
    pub fn delivery_status(
        &self,
        uuid: &str,
        options: &DeliveryStatusOptions,
    ) -> Result<DeliveryStatusResponse> {
        // Percent-encode the ISO 8601 timestamp for query-string use. ISO
        // timestamps contain `:` and `+` which must be encoded in a query
        // value; every other byte we expect here is already safe.
        let suffix = match options.poll_from.as_deref() {
            Some(ts) if !ts.is_empty() => {
                let mut encoded = String::with_capacity(ts.len() + 8);
                for b in ts.bytes() {
                    match b {
                        b'0'..=b'9'
                        | b'A'..=b'Z'
                        | b'a'..=b'z'
                        | b'-' | b'_' | b'.' | b'~' => encoded.push(b as char),
                        _ => encoded.push_str(&format!("%{:02X}", b)),
                    }
                }
                format!("?pollFrom={}", encoded)
            }
            _ => String::new(),
        };
        self.http.request(
            "GET",
            &format!("/api/v1/upload-sessions/{}/delivery-status{}", uuid, suffix),
            None,
        )
    }
}
