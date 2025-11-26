use crate::models;
use serde::{Deserialize, Serialize};

/// FiatDepositStatus : Statuses for a FiatDeposit.
/// Statuses for a FiatDeposit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FiatDepositStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Settled")]
    Settled,
}

impl std::fmt::Display for FiatDepositStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Failed => write!(f, "Failed"),
            Self::Settled => write!(f, "Settled"),
        }
    }
}

impl Default for FiatDepositStatus {
    fn default() -> FiatDepositStatus {
        Self::Pending
    }
}
