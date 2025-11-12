//! Onboarding-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Fiat option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatOption {
    /// Fiat currency code
    #[serde(rename = "FiatCurrencyCode")]
    pub fiat_currency_code: FiatCurrencyCode,
}

/// Hosted onboarding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedOnboardingRequest {
    /// Return URL
    #[serde(rename = "ReturnURL")]
    pub return_url: ReturnURL,
    /// Fiat options
    #[serde(rename = "FiatOptions")]
    pub fiat_options: Vec<FiatOption>,
    /// Form (optional)
    #[serde(rename = "Form")]
    pub form: Option<serde_json::Value>,
    /// Metadata (optional)
    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Hosted session response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedSessionResponse {
    /// Hosted URL
    #[serde(rename = "HostedURL")]
    pub hosted_url: String,
    /// Form schema (optional)
    #[serde(rename = "FormSchema")]
    pub form_schema: Option<serde_json::Value>,
}

/// Prefill document upload URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefillDocumentUploadURLResponse {
    /// Presigned URL
    #[serde(rename = "PresignedURL")]
    pub presigned_url: String,
    /// Expires at
    #[serde(rename = "ExpiresAt")]
    pub expires_at: DateTime,
}

