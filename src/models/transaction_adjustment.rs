use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionAdjustment {
    /// ID of the transaction being adjusted.
    #[serde(rename = "AdjustedTransactionID")]
    pub adjusted_transaction_id: uuid::Uuid,
    /// A unique identifier linking related transactions that are involved in a single Adjustment. For Adjustments involving multiple transactions, this ID is shared across all associated records.
    #[serde(rename = "AdjustmentID")]
    pub adjustment_id: uuid::Uuid,
    /// Why the adjustment was made: * ExchangeRateCorrection
    #[serde(rename = "Reason")]
    pub reason: String,
}

impl TransactionAdjustment {
    pub fn new(
        adjusted_transaction_id: uuid::Uuid,
        adjustment_id: uuid::Uuid,
        reason: String,
    ) -> TransactionAdjustment {
        TransactionAdjustment {
            adjusted_transaction_id,
            adjustment_id,
            reason,
        }
    }
}
