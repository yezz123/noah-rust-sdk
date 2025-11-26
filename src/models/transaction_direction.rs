use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionDirection : Whether the transactions was a credit (in) or a debit (out) in your account
/// Whether the transactions was a credit (in) or a debit (out) in your account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionDirection {
    #[serde(rename = "In")]
    In,
    #[serde(rename = "Out")]
    Out,
}

impl std::fmt::Display for TransactionDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::In => write!(f, "In"),
            Self::Out => write!(f, "Out"),
        }
    }
}

impl Default for TransactionDirection {
    fn default() -> TransactionDirection {
        Self::In
    }
}
