use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceResponse {
    /// Type of account:   * Current
    #[serde(rename = "AccountType")]
    pub account_type: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "Available")]
    pub available: String,
    #[serde(rename = "Total")]
    pub total: String,
}

impl BalanceResponse {
    pub fn new(
        account_type: String,
        crypto_currency: String,
        available: String,
        total: String,
    ) -> BalanceResponse {
        BalanceResponse {
            account_type,
            crypto_currency,
            available,
            total,
        }
    }
}
