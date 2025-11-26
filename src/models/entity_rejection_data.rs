use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRejectionData {
    #[serde(rename = "RejectType")]
    pub reject_type: RejectType,
    #[serde(rename = "PublicComment", skip_serializing_if = "Option::is_none")]
    pub public_comment: Option<String>,
}

impl EntityRejectionData {
    pub fn new(reject_type: RejectType) -> EntityRejectionData {
        EntityRejectionData {
            reject_type,
            public_comment: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RejectType {
    #[serde(rename = "Final")]
    Final,
    #[serde(rename = "Retry")]
    Retry,
}

impl Default for RejectType {
    fn default() -> RejectType {
        Self::Final
    }
}
