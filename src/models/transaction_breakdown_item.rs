use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionBreakdownItem {
    /// What does the breakdown item amount refer to: * ChannelFee: amount withheld by the system from the fiat payment * BusinessFee: amount to withheld on behalf of the business over its customers transactions * Remaining: amount withheld by the system from the fiat payment
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Amount")]
    pub amount: String,
}

impl TransactionBreakdownItem {
    pub fn new(r#type: String, amount: String) -> TransactionBreakdownItem {
        TransactionBreakdownItem { r#type, amount }
    }
}
