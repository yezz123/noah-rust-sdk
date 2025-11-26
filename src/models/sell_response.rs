use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SellResponse {
    #[serde(rename = "Transaction")]
    pub transaction: Box<models::Transaction>,
}

impl SellResponse {
    pub fn new(transaction: models::Transaction) -> SellResponse {
        SellResponse {
            transaction: Box::new(transaction),
        }
    }
}
