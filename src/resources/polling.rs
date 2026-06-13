//! Long-polling for image downloads.

use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{PollImage, PollOptions, PollResult};
use std::sync::Arc;

/// Poll for and download images from a long-polling session.
///
/// Access via [`Apertur::polling()`](crate::Apertur::polling).
pub struct Polling {
    http: Arc<HttpClient>,
}

impl Polling {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List pending images available for download.
    pub fn list(&self, uuid: &str) -> Result<PollResult> {
        self.http.request(
            "GET",
            &format!("/api/v1/upload-sessions/{}/poll", uuid),
            None,
        )
    }

    /// Download an image by its ID.
    ///
    /// Returns the raw image bytes.
    pub fn download(&self, uuid: &str, image_id: &str) -> Result<Vec<u8>> {
        self.http.request_raw(
            "GET",
            &format!("/api/v1/upload-sessions/{}/images/{}", uuid, image_id),
        )
    }

    /// Acknowledge receipt of an image, removing it from the poll queue.
    pub fn ack(&self, uuid: &str, image_id: &str) -> Result<()> {
        self.http.request_empty(
            "POST",
            &format!(
                "/api/v1/upload-sessions/{}/images/{}/ack",
                uuid, image_id
            ),
            None,
        )
    }

    /// Continuously poll for new images, download them, and pass them to a handler.
    ///
    /// This method blocks the current thread, polling at the interval specified in
    /// `options` (default: 3 seconds). For each new image, the handler receives
    /// the image metadata and the downloaded bytes. After the handler returns
    /// successfully, the image is automatically acknowledged.
    ///
    /// The loop runs indefinitely. To stop it, the handler should return an error
    /// or the calling code should run this in a separate thread and terminate it
    /// externally.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The session UUID.
    /// * `handler` - A closure called for each image with `(&PollImage, &[u8]) -> Result<()>`.
    /// * `options` - Polling options (interval).
    pub fn poll_and_process<F>(
        &self,
        uuid: &str,
        handler: F,
        options: &PollOptions,
    ) -> Result<()>
    where
        F: Fn(&PollImage, &[u8]) -> Result<()>,
    {
        loop {
            let result = self.list(uuid)?;

            for image in &result.images {
                let data = self.download(uuid, &image.id)?;
                handler(image, &data)?;
                self.ack(uuid, &image.id)?;
            }

            std::thread::sleep(options.interval);
        }
    }
}
