use crate::models;
use serde::{Deserialize, Serialize};

/// ProcessingTier : Processing tier for a payment method, this may affect the settlement time of the transaction.   * Standard: it's the payment processor method standard option, for BankACH it means ACH next day settlement.  * Priority: it's the payment processor method priority option, for BankACH it means ACH same day settlement.
/// Processing tier for a payment method, this may affect the settlement time of the transaction.   * Standard: it's the payment processor method standard option, for BankACH it means ACH next day settlement.  * Priority: it's the payment processor method priority option, for BankACH it means ACH same day settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProcessingTier {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "Priority")]
    Priority,
}

impl std::fmt::Display for ProcessingTier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::Priority => write!(f, "Priority"),
        }
    }
}

impl Default for ProcessingTier {
    fn default() -> ProcessingTier {
        Self::Standard
    }
}
