use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualFinancialsInput {
    /// What's your expected annual deposit?
    #[serde(rename = "AnnualDeposit", skip_serializing_if = "Option::is_none")]
    pub annual_deposit: Option<AnnualDeposit>,
}

impl IndividualFinancialsInput {
    pub fn new() -> IndividualFinancialsInput {
        IndividualFinancialsInput {
            annual_deposit: None,
        }
    }
}
/// What's your expected annual deposit?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnnualDeposit {
    #[serde(rename = "LessThan5k")]
    LessThan5k,
    #[serde(rename = "5kTo50k")]
    Variant5kTo50k,
    #[serde(rename = "50kTo150k")]
    Variant50kTo150k,
    #[serde(rename = "MoreThan150k")]
    MoreThan150k,
}

impl Default for AnnualDeposit {
    fn default() -> AnnualDeposit {
        Self::LessThan5k
    }
}
