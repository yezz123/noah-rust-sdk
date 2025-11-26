use crate::models;
use serde::{Deserialize, Serialize};

/// FiatOption : Fiat option for a customer to be onboarded with.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatOption {
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrencyCode")]
    pub fiat_currency_code: String,
}

impl FiatOption {
    /// Fiat option for a customer to be onboarded with.
    pub fn new(fiat_currency_code: String) -> FiatOption {
        FiatOption { fiat_currency_code }
    }
}
