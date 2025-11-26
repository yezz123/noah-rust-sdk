use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestExtensionItem {
    /// The full path of the field with error.
    #[serde(rename = "Field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// What reason the field has error, refers to: * IntegrityValidation * SchemaValidation
    #[serde(rename = "Reason")]
    pub reason: String,
    /// A more detailed description of the problem or the fix.
    #[serde(rename = "Description")]
    pub description: String,
}

impl RequestExtensionItem {
    pub fn new(reason: String, description: String) -> RequestExtensionItem {
        RequestExtensionItem {
            field: None,
            reason,
            description,
        }
    }
}
