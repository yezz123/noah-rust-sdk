use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatDepositSimulateRequest {
    /// Existing payment method id to be used.
    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: String,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
}

impl FiatDepositSimulateRequest {
    pub fn new(
        payment_method_id: String,
        fiat_amount: String,
        fiat_currency: String,
    ) -> FiatDepositSimulateRequest {
        FiatDepositSimulateRequest {
            payment_method_id,
            fiat_amount,
            fiat_currency,
        }
    }
}
