use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCreateRequest {
    #[serde(rename = "Trigger")]
    pub trigger: Box<models::DepositSourceTriggerInput>,
    #[serde(rename = "Actions")]
    pub actions: Vec<models::RuleCreateRequestActionsInner>,
    #[serde(rename = "Expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// If True, a rule can be executed repeatedly. Default is False.
    #[serde(rename = "Permanent", skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    /// A string which must be unique each time a new transaction is created, like a UUID or operation sequence number. Request can be idempotently retried by using the same Nonce.
    #[serde(rename = "Nonce")]
    pub nonce: String,
}

impl RuleCreateRequest {
    pub fn new(
        trigger: models::DepositSourceTriggerInput,
        actions: Vec<models::RuleCreateRequestActionsInner>,
        nonce: String,
    ) -> RuleCreateRequest {
        RuleCreateRequest {
            trigger: Box::new(trigger),
            actions,
            expiry: None,
            permanent: None,
            nonce,
        }
    }
}
