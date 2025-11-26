use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// Type of error: * InvalidMessage * Unexpected * ResourceNotFound * Unauthorized * Forbidden * InsufficientBalance
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A unique reference that identifies the specific occurrence of the problem
    #[serde(rename = "Instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Action taking place which resulted in error.
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Details about the error.
    #[serde(rename = "Detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "Extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Box<models::ErrorExtensions>>,
    #[serde(rename = "RequestExtension", skip_serializing_if = "Option::is_none")]
    pub request_extension: Option<Box<models::RequestExtension>>,
    #[serde(rename = "DenyExtension", skip_serializing_if = "Option::is_none")]
    pub deny_extension: Option<Vec<models::DenyExtensionItem>>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            r#type: None,
            instance: None,
            action: None,
            detail: None,
            extensions: None,
            request_extension: None,
            deny_extension: None,
        }
    }
}
