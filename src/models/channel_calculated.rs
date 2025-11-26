use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelCalculated {
    #[serde(rename = "TotalFee")]
    pub total_fee: String,
}

impl ChannelCalculated {
    pub fn new(total_fee: String) -> ChannelCalculated {
        ChannelCalculated { total_fee }
    }
}
