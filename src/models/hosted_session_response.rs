use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostedSessionResponse {
    /// URL to redirect the customer to hosted session.
    #[serde(rename = "HostedURL")]
    pub hosted_url: String,
    #[serde(rename = "FormSchema", skip_serializing_if = "Option::is_none")]
    pub form_schema: Option<Box<models::FormSchema>>,
}

impl HostedSessionResponse {
    pub fn new(hosted_url: String) -> HostedSessionResponse {
        HostedSessionResponse {
            hosted_url,
            form_schema: None,
        }
    }
}
