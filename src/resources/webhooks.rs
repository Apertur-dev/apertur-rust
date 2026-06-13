//! Event webhook management.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{
    ListParams, Webhook, WebhookCreateConfig, WebhookDeliveriesResult, WebhookUpdateConfig,
};
use std::sync::Arc;

/// Manage event webhooks for a project.
///
/// Access via [`Apertur::webhooks()`](crate::Apertur::webhooks).
pub struct Webhooks {
    http: Arc<HttpClient>,
}

impl Webhooks {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all webhooks for a project.
    pub fn list(&self, project_id: &str) -> Result<Vec<Webhook>> {
        self.http.request(
            "GET",
            &format!("/api/v1/projects/{}/webhooks", project_id),
            None,
        )
    }

    /// Create a new webhook.
    pub fn create(
        &self,
        project_id: &str,
        config: &WebhookCreateConfig,
    ) -> Result<Webhook> {
        let body = serde_json::to_value(config)?;
        self.http.request(
            "POST",
            &format!("/api/v1/projects/{}/webhooks", project_id),
            Some(&body),
        )
    }

    /// Update an existing webhook.
    pub fn update(
        &self,
        project_id: &str,
        webhook_id: &str,
        config: &WebhookUpdateConfig,
    ) -> Result<Webhook> {
        let body = serde_json::to_value(config)?;
        self.http.request(
            "PATCH",
            &format!(
                "/api/v1/projects/{}/webhooks/{}",
                project_id, webhook_id
            ),
            Some(&body),
        )
    }

    /// Delete a webhook.
    pub fn delete(&self, project_id: &str, webhook_id: &str) -> Result<()> {
        self.http.request_empty(
            "DELETE",
            &format!(
                "/api/v1/projects/{}/webhooks/{}",
                project_id, webhook_id
            ),
            None,
        )
    }

    /// Send a test event to a webhook.
    pub fn test(
        &self,
        project_id: &str,
        webhook_id: &str,
    ) -> Result<serde_json::Value> {
        self.http.request(
            "POST",
            &format!(
                "/api/v1/projects/{}/webhooks/{}/test",
                project_id, webhook_id
            ),
            None,
        )
    }

    /// List delivery attempts for a webhook.
    pub fn deliveries(
        &self,
        project_id: &str,
        webhook_id: &str,
        params: &ListParams,
    ) -> Result<WebhookDeliveriesResult> {
        let mut qs = Vec::new();
        if let Some(page) = params.page {
            qs.push(format!("page={}", page));
        }
        if let Some(limit) = params.limit {
            qs.push(format!("limit={}", limit));
        }
        let suffix = if qs.is_empty() {
            String::new()
        } else {
            format!("?{}", qs.join("&"))
        };
        self.http.request(
            "GET",
            &format!(
                "/api/v1/projects/{}/webhooks/{}/deliveries{}",
                project_id, webhook_id, suffix
            ),
            None,
        )
    }

    /// Retry a failed delivery attempt.
    pub fn retry_delivery(
        &self,
        project_id: &str,
        webhook_id: &str,
        delivery_id: &str,
    ) -> Result<serde_json::Value> {
        self.http.request(
            "POST",
            &format!(
                "/api/v1/projects/{}/webhooks/{}/deliveries/{}/retry",
                project_id, webhook_id, delivery_id
            ),
            None,
        )
    }
}
