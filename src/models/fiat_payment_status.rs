use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FiatPaymentStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Successful")]
    Successful,
}

impl std::fmt::Display for FiatPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Failed => write!(f, "Failed"),
            Self::Successful => write!(f, "Successful"),
        }
    }
}

impl Default for FiatPaymentStatus {
    fn default() -> FiatPaymentStatus {
        Self::Pending
    }
}
