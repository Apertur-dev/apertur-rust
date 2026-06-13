//! Delivery destination management.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{Destination, DestinationCreateConfig, DestinationTestResult, DestinationUpdateConfig};
use std::sync::Arc;

/// Manage delivery destinations for a project.
///
/// Access via [`Apertur::destinations()`](crate::Apertur::destinations).
pub struct Destinations {
    http: Arc<HttpClient>,
}

impl Destinations {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all destinations for a project.
    pub fn list(&self, project_id: &str) -> Result<Vec<Destination>> {
        self.http.request(
            "GET",
            &format!("/api/v1/projects/{}/destinations", project_id),
            None,
        )
    }

    /// Create a new destination.
    pub fn create(
        &self,
        project_id: &str,
        config: &DestinationCreateConfig,
    ) -> Result<Destination> {
        let body = serde_json::to_value(config)?;
        self.http.request(
            "POST",
            &format!("/api/v1/projects/{}/destinations", project_id),
            Some(&body),
        )
    }

    /// Update an existing destination.
    pub fn update(
        &self,
        project_id: &str,
        dest_id: &str,
        config: &DestinationUpdateConfig,
    ) -> Result<Destination> {
        let body = serde_json::to_value(config)?;
        self.http.request(
            "PATCH",
            &format!("/api/v1/projects/{}/destinations/{}", project_id, dest_id),
            Some(&body),
        )
    }

    /// Delete a destination.
    pub fn delete(&self, project_id: &str, dest_id: &str) -> Result<()> {
        self.http.request_empty(
            "DELETE",
            &format!("/api/v1/projects/{}/destinations/{}", project_id, dest_id),
            None,
        )
    }

    /// Test a destination by sending a test payload.
    pub fn test(&self, project_id: &str, dest_id: &str) -> Result<DestinationTestResult> {
        self.http.request(
            "POST",
            &format!(
                "/api/v1/projects/{}/destinations/{}/test",
                project_id, dest_id
            ),
            None,
        )
    }
}
