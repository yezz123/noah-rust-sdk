use crate::models;
use serde::{Deserialize, Serialize};

/// RequestExtension : Additional information about the request fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestExtension {
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<models::RequestExtensionItem>>,
}

impl RequestExtension {
    /// Additional information about the request fields.
    pub fn new() -> RequestExtension {
        RequestExtension { body: None }
    }
}
