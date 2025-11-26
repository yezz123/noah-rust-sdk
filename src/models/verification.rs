use crate::models;
use serde::{Deserialize, Serialize};

/// Verification : Verification model and status of the customer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Status")]
    pub status: Status,
}

impl Verification {
    /// Verification model and status of the customer
    pub fn new(model: String, status: Status) -> Verification {
        Verification { model, status }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Declined")]
    Declined,
}

impl Default for Status {
    fn default() -> Status {
        Self::Approved
    }
}
