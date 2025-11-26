use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnchainDepositToPaymentMethodResponse {
    /// Conditions that trigger the rule.
    #[serde(rename = "Conditions")]
    pub conditions: Vec<models::DepositSourceTriggerCondition>,
    /// Address for transfer
    #[serde(rename = "SourceAddress", skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
}

impl OnchainDepositToPaymentMethodResponse {
    pub fn new(
        conditions: Vec<models::DepositSourceTriggerCondition>,
    ) -> OnchainDepositToPaymentMethodResponse {
        OnchainDepositToPaymentMethodResponse {
            conditions,
            source_address: None,
        }
    }
}
