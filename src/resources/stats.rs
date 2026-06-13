//! Account statistics.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::Stats;
use std::sync::Arc;

/// Retrieve account-level statistics.
///
/// Access via [`Apertur::stats()`](crate::Apertur::stats).
pub struct StatsResource {
    http: Arc<HttpClient>,
}

impl StatsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Get account statistics including session counts, upload totals, and top projects.
    pub fn get(&self) -> Result<Stats> {
        self.http.request("GET", "/api/v1/stats", None)
    }
}
