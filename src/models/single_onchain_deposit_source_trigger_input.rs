use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleOnchainDepositSourceTriggerInput {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Conditions that trigger the rule.
    #[serde(rename = "Conditions")]
    pub conditions: Vec<models::OnchainDepositSourceTriggerConditionInput>,
    /// Address for transfer
    #[serde(rename = "SourceAddress")]
    pub source_address: String,
    #[serde(rename = "Expiry")]
    pub expiry: String,
    /// A string which must be unique each time a new transaction is created, like a UUID or operation sequence number. Request can be idempotently retried by using the same Nonce.
    #[serde(rename = "Nonce")]
    pub nonce: String,
}

impl SingleOnchainDepositSourceTriggerInput {
    pub fn new(
        r#type: Type,
        conditions: Vec<models::OnchainDepositSourceTriggerConditionInput>,
        source_address: String,
        expiry: String,
        nonce: String,
    ) -> SingleOnchainDepositSourceTriggerInput {
        SingleOnchainDepositSourceTriggerInput {
            r#type,
            conditions,
            source_address,
            expiry,
            nonce,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SingleOnchainDepositSourceTriggerInput")]
    SingleOnchainDepositSourceTriggerInput,
}

impl Default for Type {
    fn default() -> Type {
        Self::SingleOnchainDepositSourceTriggerInput
    }
}
