use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepDecimalOperation {
    #[serde(rename = "Operator")]
    pub operator: Operator,
    /// Decimal or output reference.
    #[serde(rename = "Value")]
    pub value: String,
}

impl StepDecimalOperation {
    pub fn new(operator: Operator, value: String) -> StepDecimalOperation {
        StepDecimalOperation { operator, value }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "Add")]
    Add,
    #[serde(rename = "Sub")]
    Sub,
    #[serde(rename = "Mul")]
    Mul,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Add
    }
}
