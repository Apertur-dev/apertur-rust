//! Upload history listing.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{ListParams, UploadPage, UploadRecord};
use std::sync::Arc;

/// List and query upload records.
///
/// Access via [`Apertur::uploads()`](crate::Apertur::uploads).
pub struct Uploads {
    http: Arc<HttpClient>,
}

impl Uploads {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List uploads with pagination.
    pub fn list(&self, params: &ListParams) -> Result<UploadPage> {
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
            .request("GET", &format!("/api/v1/uploads{}", suffix), None)
    }

    /// Get recent uploads.
    pub fn recent(&self, params: &ListParams) -> Result<Vec<UploadRecord>> {
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
            .request("GET", &format!("/api/v1/uploads/recent{}", suffix), None)
    }
}
