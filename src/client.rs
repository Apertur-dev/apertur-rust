//! Main Apertur client.

use crate::error::{AperturError, Result};
use crate::http::HttpClient;
use crate::resources::destinations::Destinations;
use crate::resources::encryption::Encryption;
use crate::resources::keys::Keys;
use crate::resources::polling::Polling;
use crate::resources::sessions::Sessions;
use crate::resources::stats::StatsResource;
use crate::resources::upload::Upload;
use crate::resources::uploads::Uploads;
use crate::resources::webhooks::Webhooks;
use crate::types::AperturConfig;
use std::sync::Arc;

const DEFAULT_BASE_URL: &str = "https://api.aptr.ca";
const SANDBOX_BASE_URL: &str = "https://sandbox.api.aptr.ca";

/// The environment targeted by the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Production environment.
    Live,
    /// Sandbox / test environment.
    Test,
}

/// The main entry point for the Apertur SDK.
///
/// Create an instance with [`Apertur::new`] (auto-detect environment from key prefix)
/// or [`Apertur::with_config`] for full control.
///
/// # Example
///
/// ```no_run
/// use apertur_sdk::Apertur;
///
/// let client = Apertur::new("aptr_test_abc123").unwrap();
/// let session = client.sessions().create(&Default::default()).unwrap();
/// println!("Session UUID: {}", session.uuid);
/// ```
pub struct Apertur {
    http: Arc<HttpClient>,
    /// The environment this client targets, inferred from the API key prefix.
    env: Environment,
}

impl Apertur {
    /// Create a new client with the given API key or OAuth token.
    ///
    /// The environment and base URL are auto-detected from the key prefix:
    /// - Keys starting with `aptr_test_` target the sandbox at `https://sandbox.api.aptr.ca`.
    /// - All other keys target the live API at `https://api.aptr.ca`.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    pub fn new(api_key: &str) -> Result<Self> {
        Self::with_config(AperturConfig {
            api_key: api_key.to_string(),
            base_url: None,
        })
    }

    /// Create a new client with explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty or the HTTP client cannot be initialized.
    pub fn with_config(config: AperturConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(AperturError::Validation {
                status_code: 0,
                code: "INVALID_CONFIG".to_string(),
                message: "API key must not be empty".to_string(),
            });
        }

        let env = if config.api_key.starts_with("aptr_test_") {
            Environment::Test
        } else {
            Environment::Live
        };

        let base_url = config.base_url.unwrap_or_else(|| {
            match env {
                Environment::Test => SANDBOX_BASE_URL.to_string(),
                Environment::Live => DEFAULT_BASE_URL.to_string(),
            }
        });

        let http = Arc::new(HttpClient::new(&base_url, &config.api_key)?);

        Ok(Self { http, env })
    }

    /// Returns the environment this client targets.
    pub fn env(&self) -> Environment {
        self.env
    }

    /// Access session management operations.
    pub fn sessions(&self) -> Sessions {
        Sessions::new(Arc::clone(&self.http))
    }

    /// Access image upload operations.
    pub fn upload(&self) -> Upload {
        Upload::new(Arc::clone(&self.http))
    }

    /// Access upload history listing.
    pub fn uploads(&self) -> Uploads {
        Uploads::new(Arc::clone(&self.http))
    }

    /// Access long-polling operations.
    pub fn polling(&self) -> Polling {
        Polling::new(Arc::clone(&self.http))
    }

    /// Access destination management operations.
    pub fn destinations(&self) -> Destinations {
        Destinations::new(Arc::clone(&self.http))
    }

    /// Access API key management operations.
    pub fn keys(&self) -> Keys {
        Keys::new(Arc::clone(&self.http))
    }

    /// Access webhook management operations.
    pub fn webhooks(&self) -> Webhooks {
        Webhooks::new(Arc::clone(&self.http))
    }

    /// Access encryption key retrieval.
    pub fn encryption(&self) -> Encryption {
        Encryption::new(Arc::clone(&self.http))
    }

    /// Access account statistics.
    pub fn stats(&self) -> StatsResource {
        StatsResource::new(Arc::clone(&self.http))
    }
}
