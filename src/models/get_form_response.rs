use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFormResponse {
    #[serde(rename = "FormSchema", skip_serializing_if = "Option::is_none")]
    pub form_schema: Option<Box<models::FormSchema>>,
}

impl GetFormResponse {
    pub fn new() -> GetFormResponse {
        GetFormResponse { form_schema: None }
    }
}
