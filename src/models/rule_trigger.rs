use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RuleTrigger {
    #[serde(rename = "DepositDestinationTrigger")]
    DepositDestinationTrigger(Box<models::DepositDestinationTrigger>),
    #[serde(rename = "DepositSourceTrigger")]
    DepositSourceTrigger(Box<models::DepositSourceTrigger>),
}

impl Default for RuleTrigger {
    fn default() -> Self {
        Self::DepositDestinationTrigger(Default::default())
    }
}
