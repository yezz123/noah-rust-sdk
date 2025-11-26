use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetChannelsResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::Channel>,
    #[serde(rename = "PageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetChannelsResponse {
    pub fn new(items: Vec<models::Channel>) -> GetChannelsResponse {
        GetChannelsResponse {
            items,
            page_token: None,
        }
    }
}
