use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositSourceTriggerInput {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Conditions that trigger the rule.
    #[serde(rename = "Conditions")]
    pub conditions: Vec<models::DepositSourceTriggerConditionInput>,
    /// Address for transfer
    #[serde(rename = "SourceAddress")]
    pub source_address: String,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
}

impl DepositSourceTriggerInput {
    pub fn new(
        r#type: Type,
        conditions: Vec<models::DepositSourceTriggerConditionInput>,
        source_address: String,
        customer_id: String,
    ) -> DepositSourceTriggerInput {
        DepositSourceTriggerInput {
            r#type,
            conditions,
            source_address,
            customer_id,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DepositSourceTriggerInput")]
    DepositSourceTriggerInput,
}

impl Default for Type {
    fn default() -> Type {
        Self::DepositSourceTriggerInput
    }
}
