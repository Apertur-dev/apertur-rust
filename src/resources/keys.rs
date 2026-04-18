//! API key management.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{ApiKey, ApiKeyCreateOptions, ApiKeyCreateResult, ApiKeyUpdateOptions, KeyDestinations};
use std::sync::Arc;

/// Manage API keys for a project.
///
/// Access via [`Apertur::keys()`](crate::Apertur::keys).
pub struct Keys {
    http: Arc<HttpClient>,
}

impl Keys {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all API keys for a project.
    pub fn list(&self, project_id: &str) -> Result<Vec<ApiKey>> {
        self.http.request(
            "GET",
            &format!("/api/v1/projects/{}/keys", project_id),
            None,
        )
    }

    /// Create a new API key.
    ///
    /// The plaintext key is only returned once in the response.
    pub fn create(
        &self,
        project_id: &str,
        options: &ApiKeyCreateOptions,
    ) -> Result<ApiKeyCreateResult> {
        let body = serde_json::to_value(options)?;
        self.http.request(
            "POST",
            &format!("/api/v1/projects/{}/keys", project_id),
            Some(&body),
        )
    }

    /// Update an existing API key.
    pub fn update(
        &self,
        project_id: &str,
        key_id: &str,
        options: &ApiKeyUpdateOptions,
    ) -> Result<ApiKey> {
        let body = serde_json::to_value(options)?;
        self.http.request(
            "PATCH",
            &format!("/api/v1/projects/{}/keys/{}", project_id, key_id),
            Some(&body),
        )
    }

    /// Delete an API key.
    pub fn delete(&self, project_id: &str, key_id: &str) -> Result<()> {
        self.http.request_empty(
            "DELETE",
            &format!("/api/v1/projects/{}/keys/{}", project_id, key_id),
            None,
        )
    }

    /// Set destinations and long-polling configuration for an API key.
    pub fn set_destinations(
        &self,
        key_id: &str,
        dest_ids: &[String],
        long_polling: bool,
    ) -> Result<KeyDestinations> {
        let body = serde_json::json!({
            "destination_ids": dest_ids,
            "long_polling_enabled": long_polling,
        });
        self.http.request(
            "PUT",
            &format!("/api/v1/keys/{}/destinations", key_id),
            Some(&body),
        )
    }
}
