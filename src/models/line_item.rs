use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    /// Description of the line item.
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Quantity")]
    pub quantity: String,
    #[serde(rename = "UnitAmount")]
    pub unit_amount: String,
    #[serde(rename = "TotalAmount")]
    pub total_amount: String,
}

impl LineItem {
    pub fn new(
        description: String,
        quantity: String,
        unit_amount: String,
        total_amount: String,
    ) -> LineItem {
        LineItem {
            description,
            quantity,
            unit_amount,
            total_amount,
        }
    }
}
