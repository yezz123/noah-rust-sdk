use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SumSubToken {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Sumsub token to share applicant.
    #[serde(rename = "Token")]
    pub token: String,
}

impl SumSubToken {
    pub fn new(r#type: Type, token: String) -> SumSubToken {
        SumSubToken { r#type, token }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SumSubToken")]
    SumSubToken,
}

impl Default for Type {
    fn default() -> Type {
        Self::SumSubToken
    }
}
