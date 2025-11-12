//! Balance-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    /// Account type
    #[serde(rename = "AccountType")]
    pub account_type: AccountType,
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Available amount
    #[serde(rename = "Available")]
    pub available: PositiveDecimal,
    /// Total amount
    #[serde(rename = "Total")]
    pub total: PositiveDecimal,
}

/// Get balances response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalancesResponse {
    /// List of balances
    #[serde(rename = "Items")]
    pub items: Vec<BalanceResponse>,
    /// Pagination token
    #[serde(rename = "PageToken")]
    pub page_token: Option<String>,
}

