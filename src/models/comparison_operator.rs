use crate::models;
use serde::{Deserialize, Serialize};

/// ComparisonOperator : Comparison operator for the field value:  * EQ (Equals)  * LTEQ (Less Than Or Equals)  * GTEQ (Greater Than Or Equals)
/// Comparison operator for the field value:  * EQ (Equals)  * LTEQ (Less Than Or Equals)  * GTEQ (Greater Than Or Equals)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComparisonOperator {
    #[serde(rename = "EQ")]
    Eq,
    #[serde(rename = "LTEQ")]
    Lteq,
    #[serde(rename = "GTEQ")]
    Gteq,
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "EQ"),
            Self::Lteq => write!(f, "LTEQ"),
            Self::Gteq => write!(f, "GTEQ"),
        }
    }
}

impl Default for ComparisonOperator {
    fn default() -> ComparisonOperator {
        Self::Eq
    }
}
