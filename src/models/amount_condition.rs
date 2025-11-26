use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmountCondition {
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: models::ComparisonOperator,
    #[serde(rename = "Value")]
    pub value: String,
}

impl AmountCondition {
    pub fn new(comparison_operator: models::ComparisonOperator, value: String) -> AmountCondition {
        AmountCondition {
            comparison_operator,
            value,
        }
    }
}
