//! Error types for the Noah SDK

use serde::{Deserialize, Serialize};
use std::fmt;

/// Main error type for the Noah SDK
#[derive(Debug, thiserror::Error)]
pub enum NoahError {
    /// HTTP client errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API error responses from the server
    #[error("API error: {0}")]
    ApiError(#[from] ApiErrorResponse),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Request validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// JSON deserialization errors
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    /// JWT signing errors
    #[error("JWT signing error: {0}")]
    JwtError(String),

    /// Other errors
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),
}

/// API error response structure matching the OpenAPI schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Error type
    #[serde(rename = "Type")]
    pub error_type: ErrorType,

    /// Unique instance identifier
    #[serde(rename = "Instance")]
    pub instance: Option<String>,

    /// Action that caused the error
    #[serde(rename = "Action")]
    pub action: Option<String>,

    /// Error details
    #[serde(rename = "Detail")]
    pub detail: Option<String>,

    /// Error extensions
    #[serde(rename = "Extensions")]
    pub extensions: Option<ErrorExtensions>,

    /// Request extension details
    #[serde(rename = "RequestExtension")]
    pub request_extension: Option<RequestExtension>,

    /// Deny extension details
    #[serde(rename = "DenyExtension")]
    pub deny_extension: Option<Vec<DenyExtensionItem>>,
}

/// Error type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ErrorType {
    InvalidMessage,
    Unexpected,
    ResourceNotFound,
    Unauthorized,
    Forbidden,
    InsufficientBalance,
}

/// Error extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExtensions {
    /// Request details
    #[serde(rename = "Request")]
    pub request: Option<serde_json::Value>,

    /// Feature flags
    #[serde(rename = "Features")]
    pub features: Option<ErrorExtensionFeatures>,
}

/// Error extension features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExtensionFeatures {
    #[serde(flatten)]
    pub features: std::collections::HashMap<String, Vec<String>>,
}

/// Request extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestExtension {
    /// Body field errors
    #[serde(rename = "Body")]
    pub body: Vec<RequestExtensionItem>,
}

/// Request extension item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestExtensionItem {
    /// Field path
    #[serde(rename = "Field")]
    pub field: String,

    /// Error reason
    #[serde(rename = "Reason")]
    pub reason: String,

    /// Error description
    #[serde(rename = "Description")]
    pub description: String,
}

/// Deny extension item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenyExtensionItem {
    /// Denial reason
    #[serde(rename = "Reason")]
    pub reason: String,

    /// Description
    #[serde(rename = "Description")]
    pub description: String,

    /// Principal
    #[serde(rename = "Principal")]
    pub principal: DenyExtensionPrincipal,
}

/// Deny extension principal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenyExtensionPrincipal {
    /// Principal ID
    #[serde(rename = "ID")]
    pub id: String,

    /// Principal type
    #[serde(rename = "Type")]
    pub principal_type: String,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_type)?;
        if let Some(ref detail) = self.detail {
            write!(f, ": {}", detail)?;
        }
        Ok(())
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::InvalidMessage => write!(f, "InvalidMessage"),
            ErrorType::Unexpected => write!(f, "Unexpected"),
            ErrorType::ResourceNotFound => write!(f, "ResourceNotFound"),
            ErrorType::Unauthorized => write!(f, "Unauthorized"),
            ErrorType::Forbidden => write!(f, "Forbidden"),
            ErrorType::InsufficientBalance => write!(f, "InsufficientBalance"),
        }
    }
}

impl std::error::Error for ApiErrorResponse {}

/// Result type alias for Noah SDK operations
pub type Result<T> = std::result::Result<T, NoahError>;

