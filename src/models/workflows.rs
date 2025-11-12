//! Workflow-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Comparison operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ComparisonOperator {
    /// Equals
    Eq,
    /// Less than or equals
    Lteq,
    /// Greater than or equals
    Gteq,
}

/// Amount condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountCondition {
    /// Comparison operator
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: ComparisonOperator,
    /// Value
    #[serde(rename = "Value")]
    pub value: PositiveDecimal,
}

/// Bank deposit to onchain address request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDepositToOnchainAddressRequest {
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Network
    #[serde(rename = "Network")]
    pub network: Network,
    /// Destination address
    #[serde(rename = "DestinationAddress")]
    pub destination_address: DestinationAddress,
}

/// Bank deposit to onchain address response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDepositToOnchainAddressResponse {
    /// Payment method ID
    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: PaymentMethodID,
    /// Payment method type
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: PaymentMethodType,
    /// Reference (optional)
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    /// Account number
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    /// Account holder name (optional)
    #[serde(rename = "AccountHolderName")]
    pub account_holder_name: Option<String>,
    /// Bank code (optional)
    #[serde(rename = "BankCode")]
    pub bank_code: Option<String>,
    /// Bank name (optional)
    #[serde(rename = "BankName")]
    pub bank_name: Option<String>,
    /// Bank address (optional)
    #[serde(rename = "BankAddress")]
    pub bank_address: Option<crate::models::customers::StreetAddress>,
}
