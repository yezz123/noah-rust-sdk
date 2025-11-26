use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPricesResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::PriceItem>,
}

impl GetPricesResponse {
    pub fn new(items: Vec<models::PriceItem>) -> GetPricesResponse {
        GetPricesResponse { items }
    }
}
