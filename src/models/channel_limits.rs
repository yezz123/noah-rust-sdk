use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelLimits {
    #[serde(rename = "MinLimit")]
    pub min_limit: String,
    #[serde(rename = "MaxLimit", skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<String>,
}

impl ChannelLimits {
    pub fn new(min_limit: String) -> ChannelLimits {
        ChannelLimits {
            min_limit,
            max_limit: None,
        }
    }
}
