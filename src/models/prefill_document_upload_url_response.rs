use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrefillDocumentUploadUrlResponse {
    /// URL to upload the document.
    #[serde(rename = "PresignedURL")]
    pub presigned_url: String,
    #[serde(rename = "ExpiresAt")]
    pub expires_at: String,
}

impl PrefillDocumentUploadUrlResponse {
    pub fn new(presigned_url: String, expires_at: String) -> PrefillDocumentUploadUrlResponse {
        PrefillDocumentUploadUrlResponse {
            presigned_url,
            expires_at,
        }
    }
}
