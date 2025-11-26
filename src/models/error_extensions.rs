use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorExtensions : Additional information about the error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorExtensions {
    /// Details about invalid request properties.
    #[serde(rename = "Request", skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    /// Details about the disabled feature flags
    #[serde(rename = "Features", skip_serializing_if = "Option::is_none")]
    pub features: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl ErrorExtensions {
    /// Additional information about the error.
    pub fn new() -> ErrorExtensions {
        ErrorExtensions {
            request: None,
            features: None,
        }
    }
}
