use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionStatus : Statuses for transactions.
/// Statuses for transactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Settled")]
    Settled,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Failed => write!(f, "Failed"),
            Self::Settled => write!(f, "Settled"),
        }
    }
}

impl Default for TransactionStatus {
    fn default() -> TransactionStatus {
        Self::Pending
    }
}
