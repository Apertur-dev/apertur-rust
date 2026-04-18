//! Request and response types for the Apertur API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Client config ──

/// Configuration for constructing an [`Apertur`](crate::Apertur) client.
#[derive(Debug, Clone)]
pub struct AperturConfig {
    /// API key (prefixed `aptr_` or `aptr_test_`) or OAuth bearer token.
    pub api_key: String,
    /// Override the base URL. By default this is auto-detected from the key prefix.
    pub base_url: Option<String>,
}

// ── Pagination ──

/// Common pagination and listing parameters.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListParams {
    /// Page number (1-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// Maximum number of items to return (used by `recent` endpoints).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

// ── Sessions ──

/// Options for creating a new upload session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionCreateOptions {
    /// IDs of destinations that should receive uploaded images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ids: Option<Vec<String>>,
    /// Enable long-polling on this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_polling: Option<bool>,
    /// Tags to attach to the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Number of hours until the session expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<u32>,
    /// ISO 8601 timestamp at which the session expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Maximum number of images that may be uploaded to this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u32>,
    /// MIME types that are accepted for uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
    /// Maximum pixel dimension (width or height) for uploaded images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_dimension: Option<u32>,
    /// Password to protect the upload session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Options for updating an existing upload session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionUpdateOptions {
    /// New expiration timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// New maximum number of images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u32>,
    /// New set of allowed MIME types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
    /// New maximum image dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_dimension: Option<u32>,
    /// New maximum image file size in megabytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_size_mb: Option<u32>,
    /// New password, or `None` to remove password protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<serde_json::Value>,
}

/// QR code rendering specifications returned when a session is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrSpecs {
    /// Base endpoint for QR code generation.
    pub endpoint: String,
    /// Supported output formats.
    pub formats: Vec<String>,
    /// Default query parameters.
    pub params: HashMap<String, String>,
}

/// Result of creating a new upload session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session UUID.
    pub uuid: String,
    /// Full URL where images can be uploaded.
    pub upload_url: String,
    /// URL to the session QR code.
    pub qr_url: String,
    /// QR code generation specifications.
    pub qr_specs: QrSpecs,
    /// Destinations attached to this session.
    pub destinations: Vec<SessionDestination>,
    /// Whether long-polling is enabled.
    pub long_polling: bool,
    /// ISO 8601 expiration timestamp.
    pub expires_at: String,
    /// Whether the session is password-protected.
    pub password_protected: bool,
    /// Environment (live or test).
    pub env: String,
}

/// A destination attached to a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDestination {
    /// Destination ID.
    pub id: String,
    /// Destination type (e.g. `"s3"`, `"webhook"`).
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Human-readable destination name.
    pub name: String,
}

/// Detailed session information returned by the `get` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetail {
    /// Session ID.
    pub id: String,
    /// Session status.
    pub status: String,
    /// ISO 8601 expiration timestamp.
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    /// Tags attached to the session.
    pub tags: Option<Vec<String>>,
    /// Maximum images per session from the API key configuration.
    #[serde(rename = "imagesPerSession")]
    pub images_per_session: Option<u32>,
    /// Effective maximum images (considering key and session limits).
    #[serde(rename = "effectiveMaxImages")]
    pub effective_max_images: Option<u32>,
    /// Effective allowed MIME types.
    #[serde(rename = "effectiveAllowedMimeTypes")]
    pub effective_allowed_mime_types: Option<Vec<String>>,
    /// Effective maximum image dimension in pixels.
    #[serde(rename = "effectiveMaxImageDimension")]
    pub effective_max_image_dimension: Option<u32>,
    /// Whether the session is password-protected.
    pub password_protected: Option<bool>,
    /// Server public key for E2E encryption.
    #[serde(rename = "serverPublicKey")]
    pub server_public_key: Option<String>,
    /// Whether E2E encryption is enabled.
    #[serde(rename = "e2eEnabled")]
    pub e2e_enabled: Option<bool>,
    /// E2E public key.
    #[serde(rename = "e2ePublicKey")]
    pub e2e_public_key: Option<String>,
    /// Whether E2E was downgraded.
    #[serde(rename = "e2eDowngraded")]
    pub e2e_downgraded: Option<bool>,
}

/// A session row returned in list and recent endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRow {
    /// Session ID.
    pub id: String,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Expiration timestamp.
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    /// Current status.
    pub status: String,
    /// Owning project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Owning project name.
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// Number of images uploaded.
    #[serde(rename = "imagesCount")]
    pub images_count: u32,
    /// Number of images delivered.
    #[serde(rename = "imagesDelivered")]
    pub images_delivered: u32,
    /// Number of images that failed delivery.
    #[serde(rename = "imagesFailed")]
    pub images_failed: u32,
    /// Number of destinations.
    #[serde(rename = "destinationsCount")]
    pub destinations_count: u32,
    /// Session tags.
    pub tags: Option<Vec<String>>,
    /// Whether long-polling is enabled.
    #[serde(rename = "longPollingEnabled")]
    pub long_polling_enabled: bool,
    /// Optional human-readable label.
    pub label: Option<String>,
    /// Environment (live or test).
    pub env: String,
}

/// Paginated list of sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPage {
    /// Session rows on this page.
    pub data: Vec<SessionRow>,
    /// Total number of sessions matching the query.
    pub total: u32,
    /// Current page number.
    pub page: u32,
    /// Number of items per page.
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    /// Total number of pages.
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

/// Options for customizing a QR code.
#[derive(Debug, Clone, Default, Serialize)]
pub struct QrOptions {
    /// Output format: `"png"`, `"svg"`, or `"jpeg"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Size in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Style: `"square"` or `"rounded"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Foreground color (hex without `#`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg: Option<String>,
    /// Background color (hex without `#`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    /// Border size in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_size: Option<u32>,
    /// Border color (hex without `#`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
}

// ── Upload ──

/// Result of uploading an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult {
    /// Upload record ID.
    pub id: String,
    /// Filename of the uploaded image.
    pub filename: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Number of destinations the image was sent to.
    pub destinations: u32,
    /// Whether long-polling is enabled for this session.
    pub long_polling: bool,
}

/// Options for uploading an image.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadOptions {
    /// Override the filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Override the MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Source identifier for the upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Session password, if the session is password-protected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Represents file data to upload.
pub enum UploadFile {
    /// A path to a file on disk.
    Path(std::path::PathBuf),
    /// In-memory bytes with a filename.
    Bytes(Vec<u8>, String),
}

// ── Uploads (list) ──

/// Breakdown of destinations by type for an upload record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadDestinationBreakdown {
    /// Destination type.
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Number of destinations of this type.
    pub count: u32,
}

/// A single upload record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRecord {
    /// Upload record ID.
    pub id: String,
    /// Original filename.
    pub filename: String,
    /// Size in bytes.
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    /// MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Upload source.
    pub source: String,
    /// Whether the image was encrypted.
    #[serde(rename = "isEncrypted")]
    pub is_encrypted: bool,
    /// Environment (live or test).
    pub env: String,
    /// Upload creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// ID of the session this upload belongs to.
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// Owning project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Owning project name.
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// Total number of destinations.
    #[serde(rename = "destinationsTotal")]
    pub destinations_total: u32,
    /// Number of destinations that received the image.
    #[serde(rename = "destinationsDelivered")]
    pub destinations_delivered: u32,
    /// Number of destinations that failed.
    #[serde(rename = "destinationsFailed")]
    pub destinations_failed: u32,
    /// Per-type destination counts.
    #[serde(rename = "destinationsBreakdown")]
    pub destinations_breakdown: Vec<UploadDestinationBreakdown>,
    /// Delivery status.
    pub status: String,
}

/// Paginated list of upload records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPage {
    /// Upload records on this page.
    pub data: Vec<UploadRecord>,
    /// Total number of records.
    pub total: u32,
    /// Current page number.
    pub page: u32,
    /// Number of items per page.
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    /// Total number of pages.
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

// ── Polling ──

/// A pending image available for polling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollImage {
    /// Image ID.
    pub id: String,
    /// Original filename.
    pub filename: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// MIME type.
    pub mime_type: String,
    /// Upload source.
    pub source: String,
    /// Upload timestamp.
    pub created_at: String,
}

/// Result of polling a session for new images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollResult {
    /// Pending images ready for download.
    pub images: Vec<PollImage>,
}

/// Options for the [`Polling::poll_and_process`](crate::resources::polling::Polling::poll_and_process) loop.
#[derive(Debug, Clone)]
pub struct PollOptions {
    /// Interval between polls. Defaults to 3 seconds.
    pub interval: std::time::Duration,
}

impl Default for PollOptions {
    fn default() -> Self {
        Self {
            interval: std::time::Duration::from_secs(3),
        }
    }
}

// ── Delivery status ──

/// Delivery status for a specific destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationDeliveryStatus {
    /// Destination ID.
    pub destination_id: String,
    /// Destination type.
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Destination name.
    pub name: String,
    /// Delivery status.
    pub status: String,
    /// Number of delivery attempts.
    pub attempts: u32,
    /// Last error message, if any.
    pub last_error: Option<String>,
}

/// Delivery status for a single upload record across all destinations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatus {
    /// Upload record ID.
    pub record_id: String,
    /// Original filename.
    pub filename: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Whether a thumbnail is available for this record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_thumbnail: Option<bool>,
    /// Per-destination delivery statuses.
    pub destinations: Vec<DestinationDeliveryStatus>,
}

/// Full response from the delivery-status endpoint.
///
/// Includes the overall session status, the per-file delivery states, and the
/// timestamp of the most recent change. Pass `last_changed` back as
/// [`DeliveryStatusOptions::poll_from`] to long-poll for the next change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatusResponse {
    /// Overall session status: `"pending"`, `"active"`, `"completed"`, or `"expired"`.
    pub status: String,
    /// Per-file delivery states.
    pub files: Vec<DeliveryStatus>,
    /// ISO 8601 timestamp of the most recent change. Use this as the next
    /// `poll_from` cursor.
    #[serde(rename = "lastChanged")]
    pub last_changed: String,
}

/// Optional parameters for [`Sessions::delivery_status`](crate::resources::sessions::Sessions::delivery_status).
///
/// When `poll_from` is set, the server long-polls for up to 5 minutes waiting
/// for a change past that cursor (new file, delivery transition, or session
/// status change). On timeout it returns the current snapshot.
///
/// The blocking `reqwest` client used here does not support per-request
/// timeouts, so callers that want to long-poll should build their
/// [`Apertur`](crate::Apertur) client with a request timeout of at least 6
/// minutes (e.g. `Duration::from_secs(360)`) before calling this endpoint,
/// so the server releases the response first under the happy path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryStatusOptions {
    /// ISO 8601 timestamp. When set, server holds the response until a change
    /// past this cursor, up to 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pollFrom")]
    pub poll_from: Option<String>,
}

// ── Destinations ──

/// A delivery destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    /// Destination ID.
    pub id: String,
    /// Destination type (e.g. `"s3"`, `"webhook"`, `"google_drive"`).
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Human-readable name.
    pub name: String,
    /// Type-specific configuration.
    pub config: serde_json::Value,
    /// Whether the destination is active.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Configuration for creating a new destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationCreateConfig {
    /// Destination type.
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Human-readable name.
    pub name: String,
    /// Type-specific configuration.
    pub config: serde_json::Value,
}

/// Configuration for updating an existing destination.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DestinationUpdateConfig {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    /// Whether the destination is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
}

/// Result of testing a destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationTestResult {
    /// Whether the test was successful.
    pub success: bool,
    /// HTTP status code from the destination, if applicable.
    pub status: Option<u16>,
    /// Error message, if any.
    pub error: Option<String>,
    /// Informational message.
    pub message: Option<String>,
}

// ── API Keys ──

/// An API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID.
    pub id: String,
    /// Key prefix (visible portion).
    pub prefix: String,
    /// Human-readable label.
    pub label: String,
    /// Environment (live or test).
    pub env: String,
    /// Whether the key is active.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Last used timestamp, or `None` if never used.
    #[serde(rename = "lastUsedAt")]
    pub last_used_at: Option<String>,
    /// Maximum images per session, or `None` for unlimited.
    #[serde(rename = "maxImages")]
    pub max_images: Option<u32>,
    /// Allowed MIME types.
    #[serde(rename = "allowedMimeTypes")]
    pub allowed_mime_types: Vec<String>,
    /// Maximum image dimension in pixels.
    #[serde(rename = "maxImageDimension")]
    pub max_image_dimension: Option<u32>,
    /// Whether long-polling is enabled.
    #[serde(rename = "longPollingEnabled")]
    pub long_polling_enabled: bool,
    /// Default destination IDs.
    #[serde(rename = "defaultDestinations")]
    pub default_destinations: Vec<String>,
    /// Allowed IP addresses.
    #[serde(rename = "allowedIps")]
    pub allowed_ips: Vec<String>,
    /// Allowed domains.
    #[serde(rename = "allowedDomains")]
    pub allowed_domains: Vec<String>,
    /// Whether TOTP is enabled.
    #[serde(rename = "totpEnabled")]
    pub totp_enabled: bool,
    /// Whether client certificate authentication is enabled.
    #[serde(rename = "clientCertEnabled")]
    pub client_cert_enabled: bool,
    /// Client certificate fingerprint.
    #[serde(rename = "clientCertFingerprint")]
    pub client_cert_fingerprint: Option<String>,
    /// Key creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Options for creating a new API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreateOptions {
    /// Human-readable label.
    pub label: String,
    /// Maximum images per session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u32>,
    /// Allowed MIME types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
    /// Maximum image dimension in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_dimension: Option<u32>,
}

/// Result of creating a new API key, including the plaintext key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreateResult {
    /// The created API key metadata.
    pub key: ApiKey,
    /// The full plaintext key (only shown once).
    #[serde(rename = "plainTextKey")]
    pub plain_text_key: String,
}

/// Options for updating an existing API key.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKeyUpdateOptions {
    /// New label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether the key is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    /// New maximum images per session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u32>,
    /// New allowed MIME types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
    /// New maximum image dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_dimension: Option<u32>,
    /// New allowed IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    /// New allowed domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
}

/// Result of setting destinations for an API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDestinations {
    /// Destinations now assigned to the key.
    pub destinations: Vec<KeyDestinationEntry>,
    /// Whether long-polling is enabled.
    #[serde(rename = "longPollingEnabled")]
    pub long_polling_enabled: bool,
}

/// A destination entry in a key-destinations response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDestinationEntry {
    /// Destination ID.
    pub id: String,
    /// Destination type.
    #[serde(rename = "type")]
    pub dest_type: String,
    /// Destination name.
    pub name: String,
    /// Whether the destination is active.
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

// ── Webhooks ──

/// An event webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    /// Webhook ID.
    pub id: String,
    /// Owning project ID.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Delivery URL.
    pub url: String,
    /// Signing secret.
    pub secret: String,
    /// Signature method.
    #[serde(rename = "signatureMethod")]
    pub signature_method: String,
    /// Event topics this webhook subscribes to.
    pub topics: Vec<String>,
    /// Whether the webhook is active.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Maximum delivery retries.
    #[serde(rename = "maxRetries")]
    pub max_retries: u32,
    /// Retry intervals in seconds.
    #[serde(rename = "retryIntervals")]
    pub retry_intervals: Vec<u32>,
    /// Number of consecutive failures before disabling.
    #[serde(rename = "disableAfterFailures")]
    pub disable_after_failures: u32,
    /// Current consecutive failure count.
    #[serde(rename = "consecutiveFailures")]
    pub consecutive_failures: u32,
    /// Custom HTTP headers sent with deliveries.
    #[serde(rename = "customHeaders")]
    pub custom_headers: HashMap<String, String>,
    /// Timestamp when the webhook was disabled, if applicable.
    #[serde(rename = "disabledAt")]
    pub disabled_at: Option<String>,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Configuration for creating a new webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCreateConfig {
    /// Delivery URL.
    pub url: String,
    /// Event topics to subscribe to.
    pub topics: Vec<String>,
    /// Signature method: `"hmac_sha256"` or `"svix"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_method: Option<String>,
    /// Maximum delivery retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    /// Retry intervals in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_intervals: Option<Vec<u32>>,
    /// Number of consecutive failures before disabling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_after_failures: Option<u32>,
    /// Custom HTTP headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<HashMap<String, String>>,
}

/// Configuration for updating an existing webhook.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookUpdateConfig {
    /// New delivery URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// New event topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Whether the webhook is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    /// New maximum retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    /// New retry intervals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_intervals: Option<Vec<u32>>,
    /// New consecutive failure threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_after_failures: Option<u32>,
    /// New custom headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A webhook delivery attempt record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    /// Delivery ID.
    pub id: String,
    /// Associated event log ID.
    #[serde(rename = "eventLogId")]
    pub event_log_id: String,
    /// Event topic.
    pub topic: String,
    /// Delivery status.
    pub status: String,
    /// Number of delivery attempts.
    pub attempts: u32,
    /// HTTP response code from the target, if any.
    #[serde(rename = "responseCode")]
    pub response_code: Option<u16>,
    /// HTTP response body from the target, if any.
    #[serde(rename = "responseBody")]
    pub response_body: Option<String>,
    /// Round-trip duration in milliseconds.
    #[serde(rename = "durationMs")]
    pub duration_ms: u64,
    /// Last error message.
    #[serde(rename = "lastError")]
    pub last_error: Option<String>,
    /// Timestamp of the next retry attempt.
    #[serde(rename = "nextRetryAt")]
    pub next_retry_at: Option<String>,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Paginated list of webhook deliveries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDeliveriesResult {
    /// Delivery records.
    pub deliveries: Vec<WebhookDelivery>,
    /// Total number of deliveries.
    pub total: u32,
    /// Current page number.
    pub page: u32,
    /// Number of items per page.
    pub limit: u32,
}

// ── Encryption ──

/// The server's public key for E2E encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerKey {
    /// PEM-encoded RSA public key.
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

/// An encrypted image payload ready for upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    /// Base64-encoded RSA-wrapped AES key.
    #[serde(rename = "encryptedKey")]
    pub encrypted_key: String,
    /// Base64-encoded initialization vector.
    pub iv: String,
    /// Base64-encoded AES-GCM ciphertext (includes auth tag).
    #[serde(rename = "encryptedData")]
    pub encrypted_data: String,
    /// Algorithm identifier.
    pub algorithm: String,
}

// ── Stats ──

/// Account-level statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Number of sessions created this month.
    #[serde(rename = "sessionsThisMonth")]
    pub sessions_this_month: u64,
    /// Total sessions ever created.
    #[serde(rename = "sessionsTotal")]
    pub sessions_total: u64,
    /// Total images uploaded.
    #[serde(rename = "imagesUploaded")]
    pub images_uploaded: u64,
    /// Total images delivered.
    #[serde(rename = "imagesDelivered")]
    pub images_delivered: u64,
    /// Delivery success rate as a percentage (0.0 - 100.0).
    #[serde(rename = "deliverySuccessRate")]
    pub delivery_success_rate: f64,
    /// Total number of projects.
    #[serde(rename = "totalProjects")]
    pub total_projects: u32,
    /// Number of active API keys.
    #[serde(rename = "activeKeys")]
    pub active_keys: u32,
    /// Top projects by session count.
    #[serde(rename = "topProjects")]
    pub top_projects: Vec<TopProject>,
}

/// A project in the top-projects list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProject {
    /// Project ID.
    pub id: String,
    /// Project name.
    pub name: String,
    /// Number of sessions.
    pub sessions: u64,
}
