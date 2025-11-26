use crate::models;
use serde::{Deserialize, Serialize};

/// BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit : Indicate in % how many low-risk, medium-risk, and high-risk customers the company has.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit {
    /// Low-risk customers
    #[serde(rename = "LowRisk")]
    pub low_risk: i32,
    /// Medium-risk customers
    #[serde(rename = "MediumRisk")]
    pub medium_risk: i32,
    /// High-risk customers
    #[serde(rename = "HighRisk")]
    pub high_risk: i32,
}

impl BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit {
    /// Indicate in % how many low-risk, medium-risk, and high-risk customers the company has.
    pub fn new(
        low_risk: i32,
        medium_risk: i32,
        high_risk: i32,
    ) -> BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit {
        BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit {
            low_risk,
            medium_risk,
            high_risk,
        }
    }
}
