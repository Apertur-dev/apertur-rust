//! HTTP client wrapper for the Apertur API.

use crate::error::{AperturError, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;

/// Low-level HTTP client that handles authentication, serialization, and error mapping.
pub struct HttpClient {
    client: Client,
    base_url: String,
    auth_header: String,
}

impl HttpClient {
    /// Create a new HTTP client targeting the given base URL with the given bearer token.
    pub fn new(base_url: &str, token: &str) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(AperturError::Http)?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_header: format!("Bearer {}", token),
        })
    }

    /// Perform a JSON API request and deserialize the response.
    ///
    /// For `DELETE` requests that return HTTP 204, this returns `T::default()` if `T`
    /// implements `Default`, or an empty JSON value otherwise. Callers that expect
    /// `()` should use [`request_empty`](Self::request_empty) instead.
    pub fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "PATCH" => self.client.patch(&url),
            "DELETE" => self.client.delete(&url),
            _ => self.client.get(&url),
        };

        builder = builder.header(AUTHORIZATION, &self.auth_header);

        if let Some(json_body) = body {
            builder = builder
                .header(CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(json_body)?);
        }

        let response = builder.send().map_err(AperturError::Http)?;
        let status = response.status();

        if status == reqwest::StatusCode::NO_CONTENT {
            // 204 responses have no body; parse from an empty JSON object.
            return serde_json::from_str("null")
                .or_else(|_| serde_json::from_str("{}"))
                .map_err(AperturError::Json);
        }

        if !status.is_success() {
            return Err(self.map_error(status.as_u16(), response));
        }

        let text = response.text().map_err(AperturError::Http)?;
        serde_json::from_str(&text).map_err(AperturError::Json)
    }

    /// Perform a request that is expected to return no body (e.g. DELETE 204).
    pub fn request_empty(
        &self,
        method: &str,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "PATCH" => self.client.patch(&url),
            "DELETE" => self.client.delete(&url),
            _ => self.client.get(&url),
        };

        builder = builder.header(AUTHORIZATION, &self.auth_header);

        if let Some(json_body) = body {
            builder = builder
                .header(CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(json_body)?);
        }

        let response = builder.send().map_err(AperturError::Http)?;
        let status = response.status();

        if !status.is_success() {
            return Err(self.map_error(status.as_u16(), response));
        }

        Ok(())
    }

    /// Perform a request and return the raw response bytes (e.g. for binary downloads).
    pub fn request_raw(&self, method: &str, path: &str) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            _ => self.client.get(&url),
        };

        builder = builder.header(AUTHORIZATION, &self.auth_header);

        let response = builder.send().map_err(AperturError::Http)?;
        let status = response.status();

        if !status.is_success() {
            return Err(self.map_error(status.as_u16(), response));
        }

        response.bytes().map(|b| b.to_vec()).map_err(AperturError::Http)
    }

    /// Perform a multipart form upload.
    pub fn request_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::blocking::multipart::Form,
        extra_headers: Option<HeaderMap>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = self
            .client
            .post(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .multipart(form);

        if let Some(headers) = extra_headers {
            for (key, value) in headers.iter() {
                builder = builder.header(key, value);
            }
        }

        let response = builder.send().map_err(AperturError::Http)?;
        let status = response.status();

        if !status.is_success() {
            return Err(self.map_error(status.as_u16(), response));
        }

        let text = response.text().map_err(AperturError::Http)?;
        serde_json::from_str(&text).map_err(AperturError::Json)
    }

    /// Perform a JSON POST with custom headers (used for encrypted uploads).
    pub fn request_json_with_headers<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &str,
        extra_headers: HeaderMap,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let mut builder = self
            .client
            .post(&url)
            .header(AUTHORIZATION, &self.auth_header)
            .header(CONTENT_TYPE, "application/json")
            .body(body.to_string());

        for (key, value) in extra_headers.iter() {
            builder = builder.header(key, value);
        }

        let response = builder.send().map_err(AperturError::Http)?;
        let status = response.status();

        if !status.is_success() {
            return Err(self.map_error(status.as_u16(), response));
        }

        let text = response.text().map_err(AperturError::Http)?;
        serde_json::from_str(&text).map_err(AperturError::Json)
    }

    /// Map an HTTP error response to an [`AperturError`].
    fn map_error(
        &self,
        status_code: u16,
        response: reqwest::blocking::Response,
    ) -> AperturError {
        let body_text = response.text().unwrap_or_default();
        let body: serde_json::Value =
            serde_json::from_str(&body_text).unwrap_or_else(|_| serde_json::json!({}));

        let message = body["message"]
            .as_str()
            .unwrap_or(&format!("HTTP {}", status_code))
            .to_string();
        let code = body["code"]
            .as_str()
            .unwrap_or("")
            .to_string();

        match status_code {
            401 => AperturError::Authentication {
                status_code,
                code,
                message,
            },
            404 => AperturError::NotFound {
                status_code,
                code,
                message,
            },
            429 => {
                let retry_after = body["retryAfter"].as_u64();
                AperturError::RateLimit {
                    status_code,
                    code,
                    message,
                    retry_after,
                }
            }
            400 => AperturError::Validation {
                status_code,
                code,
                message,
            },
            _ => AperturError::Api {
                status_code,
                code,
                message,
            },
        }
    }
}
