use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBalancesResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::BalanceResponse>,
    #[serde(rename = "PageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetBalancesResponse {
    pub fn new(items: Vec<models::BalanceResponse>) -> GetBalancesResponse {
        GetBalancesResponse {
            items,
            page_token: None,
        }
    }
}
