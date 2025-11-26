use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessFinancialsInput {
    /// What is the estimated monthly turnover for your account, including both incoming and outgoing transactions?
    #[serde(
        rename = "EstimatedMonthlyTurnover",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_monthly_turnover: Option<EstimatedMonthlyTurnover>,
    /// Please specify the planned value of one-time transaction:
    #[serde(
        rename = "EstimatedTransactionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_transaction_value: Option<EstimatedTransactionValue>,
}

impl BusinessFinancialsInput {
    pub fn new() -> BusinessFinancialsInput {
        BusinessFinancialsInput {
            estimated_monthly_turnover: None,
            estimated_transaction_value: None,
        }
    }
}
/// What is the estimated monthly turnover for your account, including both incoming and outgoing transactions?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EstimatedMonthlyTurnover {
    #[serde(rename = "UpTo15k")]
    UpTo15k,
    #[serde(rename = "UpTo50k")]
    UpTo50k,
    #[serde(rename = "UpTo100k")]
    UpTo100k,
    #[serde(rename = "UpTo500k")]
    UpTo500k,
    #[serde(rename = "Above500k")]
    Above500k,
}

impl Default for EstimatedMonthlyTurnover {
    fn default() -> EstimatedMonthlyTurnover {
        Self::UpTo15k
    }
}
/// Please specify the planned value of one-time transaction:
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EstimatedTransactionValue {
    #[serde(rename = "UpTo5k")]
    UpTo5k,
    #[serde(rename = "UpTo15k")]
    UpTo15k,
    #[serde(rename = "UpTo50k")]
    UpTo50k,
    #[serde(rename = "UpTo100k")]
    UpTo100k,
    #[serde(rename = "Above100k")]
    Above100k,
}

impl Default for EstimatedTransactionValue {
    fn default() -> EstimatedTransactionValue {
        Self::UpTo5k
    }
}
