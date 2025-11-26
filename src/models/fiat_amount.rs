use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatAmount {
    #[serde(rename = "Amount")]
    pub amount: String,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
}

impl FiatAmount {
    pub fn new(amount: String, fiat_currency: String) -> FiatAmount {
        FiatAmount {
            amount,
            fiat_currency,
        }
    }
}
