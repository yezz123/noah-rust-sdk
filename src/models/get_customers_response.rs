use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCustomersResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::Customer>,
    #[serde(rename = "PageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetCustomersResponse {
    pub fn new(items: Vec<models::Customer>) -> GetCustomersResponse {
        GetCustomersResponse {
            items,
            page_token: None,
        }
    }
}
