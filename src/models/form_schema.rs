use crate::models;
use serde::{Deserialize, Serialize};

/// FormSchema : JSON schema for the form
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormSchema {
    #[serde(rename = "$schema")]
    pub dollar_schema: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<serde_json::Value>>,
}

impl FormSchema {
    /// JSON schema for the form
    pub fn new(
        dollar_schema: String,
        r#type: String,
        properties: std::collections::HashMap<String, serde_json::Value>,
    ) -> FormSchema {
        FormSchema {
            dollar_schema,
            r#type,
            title: None,
            description: None,
            properties,
            required: None,
            all_of: None,
        }
    }
}
