use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareSellResponse {
    #[serde(rename = "TotalFee")]
    pub total_fee: String,
    #[serde(rename = "CryptoAmountEstimate")]
    pub crypto_amount_estimate: String,
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,
    /// Unique identifier for the Form Session. Form Session allows ramping using provided form data.
    #[serde(rename = "FormSessionID")]
    pub form_session_id: uuid::Uuid,
}

impl PrepareSellResponse {
    pub fn new(
        total_fee: String,
        crypto_amount_estimate: String,
        crypto_authorized_amount: String,
        form_session_id: uuid::Uuid,
    ) -> PrepareSellResponse {
        PrepareSellResponse {
            total_fee,
            crypto_amount_estimate,
            crypto_authorized_amount,
            form_session_id,
        }
    }
}
