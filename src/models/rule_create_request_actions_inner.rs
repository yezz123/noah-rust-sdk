use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RuleCreateRequestActionsInner {
    #[serde(rename = "SellActionInput")]
    SellActionInput(Box<models::SellActionInput>),
}

impl Default for RuleCreateRequestActionsInner {
    fn default() -> Self {
        Self::SellActionInput(Default::default())
    }
}
