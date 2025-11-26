use crate::models;
use serde::{Deserialize, Serialize};

/// StepDecimal : Select a fixed amount, an input or a decimal operation with the two
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepDecimal {
    /// Decimal or output reference.
    #[serde(rename = "Value")]
    pub value: String,
    /// Operations are applied to `StepDecimal.Value` in the order they appear in the array.
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<models::StepDecimalOperation>>,
}

impl StepDecimal {
    /// Select a fixed amount, an input or a decimal operation with the two
    pub fn new(value: String) -> StepDecimal {
        StepDecimal {
            value,
            operations: None,
        }
    }
}
