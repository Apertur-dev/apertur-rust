//! # apertur-sdk
//!
//! Official Rust SDK for the [Apertur](https://apertur.ca) API.
//!
//! ## Quick start
//!
//! ```no_run
//! use apertur_sdk::{Apertur, SessionCreateOptions, UploadFile};
//!
//! fn main() -> apertur_sdk::Result<()> {
//!     let client = Apertur::new("aptr_test_abc123")?;
//!
//!     // Create an upload session
//!     let session = client.sessions().create(&SessionCreateOptions::default())?;
//!     println!("Upload URL: {}", session.upload_url);
//!
//!     // Upload an image
//!     let result = client.upload().image(
//!         &session.uuid,
//!         UploadFile::Path("photo.jpg".into()),
//!         &Default::default(),
//!     )?;
//!     println!("Uploaded: {} ({} bytes)", result.filename, result.size_bytes);
//!
//!     Ok(())
//! }
//! ```

mod client;
mod crypto;
mod error;
mod http;
pub mod resources;
mod signature;
mod types;

// Re-export the main client
pub use client::{Apertur, Environment};

// Re-export configuration
pub use types::AperturConfig;

// Re-export error types
pub use error::{AperturError, Result};

// Re-export all request/response types
pub use types::{
    ApiKey, ApiKeyCreateOptions, ApiKeyCreateResult, ApiKeyUpdateOptions,
    DeliveryStatus, DeliveryStatusOptions, DeliveryStatusResponse,
    Destination, DestinationCreateConfig, DestinationDeliveryStatus,
    DestinationTestResult, DestinationUpdateConfig,
    EncryptedPayload,
    KeyDestinationEntry, KeyDestinations,
    ListParams,
    PollImage, PollOptions, PollResult,
    QrOptions, QrSpecs,
    ServerKey,
    Session, SessionCreateOptions, SessionDetail, SessionDestination, SessionPage,
    SessionRow, SessionUpdateOptions,
    Stats, TopProject,
    UploadDestinationBreakdown, UploadFile, UploadOptions, UploadPage, UploadRecord, UploadResult,
    Webhook, WebhookCreateConfig, WebhookDeliveriesResult, WebhookDelivery, WebhookUpdateConfig,
};

// Re-export signature verification functions
pub use signature::{verify_event_signature, verify_svix_signature, verify_webhook_signature};

// Re-export encryption function
pub use crypto::encrypt_image;
