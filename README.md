# apertur-sdk

Official Rust SDK for the [Apertur](https://apertur.ca) API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
apertur-sdk = "0.1"
```

## Quick start

```rust
use apertur_sdk::{Apertur, SessionCreateOptions};

fn main() -> apertur_sdk::Result<()> {
    let client = Apertur::new("aptr_test_...")?;

    // Create an upload session
    let session = client.sessions().create(&SessionCreateOptions::default())?;
    println!("Upload URL: {}", session.upload_url);

    // Upload an image
    let result = client.upload().image(
        &session.uuid,
        apertur_sdk::UploadFile::Path("photo.jpg".into()),
        &Default::default(),
    )?;
    println!("Uploaded: {} ({} bytes)", result.filename, result.size_bytes);

    Ok(())
}
```

## Authentication

The SDK uses bearer token authentication. Pass an API key (prefixed `aptr_` or `aptr_test_`) or an OAuth token.

Test keys (`aptr_test_`) automatically target the sandbox environment at `https://sandbox.api.aptr.ca`.

```rust
use apertur_sdk::{Apertur, AperturConfig};

// Simple: auto-detect environment from key prefix
let client = Apertur::new("aptr_...")?;

// Advanced: custom configuration
let client = Apertur::with_config(AperturConfig {
    api_key: "aptr_...".into(),
    base_url: Some("https://custom.api.example.com".into()),
})?;
```

## Resources

| Resource | Accessor | Description |
|----------|----------|-------------|
| Sessions | `client.sessions()` | Create and manage upload sessions |
| Upload | `client.upload()` | Upload images to sessions |
| Uploads | `client.uploads()` | List and query uploaded images |
| Polling | `client.polling()` | Poll for and download images |
| Destinations | `client.destinations()` | Manage delivery destinations |
| Keys | `client.keys()` | Manage API keys |
| Webhooks | `client.webhooks()` | Manage event webhooks |
| Encryption | `client.encryption()` | Retrieve server encryption keys |
| Stats | `client.stats()` | Account statistics |

## Webhook signature verification

```rust
use apertur_sdk::{verify_webhook_signature, verify_event_signature, verify_svix_signature};

// Image delivery webhook
let valid = verify_webhook_signature(body, signature, secret);

// Event webhook (HMAC SHA256)
let valid = verify_event_signature(body, timestamp, signature, secret);

// Event webhook (Svix)
let valid = verify_svix_signature(body, svix_id, timestamp, signature, secret);
```

## Image encryption

```rust
use apertur_sdk::encrypt_image;

let payload = encrypt_image(image_bytes, public_key_pem)?;
```

## License

MIT
