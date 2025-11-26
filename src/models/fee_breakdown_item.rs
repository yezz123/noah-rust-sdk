use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeeBreakdownItem {
    /// What fee the items refers to: * ChannelFee * BusinessFee
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Amount")]
    pub amount: String,
}

impl FeeBreakdownItem {
    pub fn new(r#type: String, amount: String) -> FeeBreakdownItem {
        FeeBreakdownItem { r#type, amount }
    }
}
