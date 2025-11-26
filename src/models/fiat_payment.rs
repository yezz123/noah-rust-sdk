use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPayment {
    #[serde(rename = "Amount")]
    pub amount: String,
    #[serde(rename = "FeeAmount")]
    pub fee_amount: String,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
    /// The ID of the deposit when the payment originated from a deposit.
    #[serde(rename = "FiatDepositID", skip_serializing_if = "Option::is_none")]
    pub fiat_deposit_id: Option<String>,
    /// The transaction ID in the payment system (e.g., IMAD for Wire, UETR for Swift, Trace Number for ACH).
    #[serde(rename = "PaymentSystemID", skip_serializing_if = "Option::is_none")]
    pub payment_system_id: Option<String>,
}

impl FiatPayment {
    pub fn new(amount: String, fee_amount: String, fiat_currency: String) -> FiatPayment {
        FiatPayment {
            amount,
            fee_amount,
            rate: None,
            fiat_currency,
            fiat_deposit_id: None,
            payment_system_id: None,
        }
    }
}
