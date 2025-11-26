use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceItem {
    /// Categorizes one or more `PaymentMethodType`s into broader groups. Useful for listing channels, displaying payment methods:  * Bank  * Card  * Identifier
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: String,
    #[serde(rename = "Rate")]
    pub rate: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "TotalFee", skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<String>,
    #[serde(rename = "BusinessFee", skip_serializing_if = "Option::is_none")]
    pub business_fee: Option<String>,
    #[serde(rename = "SourceAmount", skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<String>,
    #[serde(rename = "DestinationAmount", skip_serializing_if = "Option::is_none")]
    pub destination_amount: Option<String>,
    /// Total fee amount breakdown in cryptocurrency.
    #[serde(rename = "CryptoFeeBreakdown", skip_serializing_if = "Option::is_none")]
    pub crypto_fee_breakdown: Option<Vec<models::FeeBreakdownItem>>,
}

impl PriceItem {
    pub fn new(payment_method_category: String, rate: String, updated_at: String) -> PriceItem {
        PriceItem {
            payment_method_category,
            rate,
            updated_at,
            total_fee: None,
            business_fee: None,
            source_amount: None,
            destination_amount: None,
            crypto_fee_breakdown: None,
        }
    }
}
