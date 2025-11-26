use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::Transaction>,
    #[serde(rename = "PageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetTransactionsResponse {
    pub fn new(items: Vec<models::Transaction>) -> GetTransactionsResponse {
        GetTransactionsResponse {
            items,
            page_token: None,
        }
    }
}
