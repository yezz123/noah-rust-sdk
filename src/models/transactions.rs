//! Transaction-related models

use crate::models::common::*;
use crate::models::payment_methods::PaymentMethod;
use serde::{Deserialize, Serialize};

/// Fiat payment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum FiatPaymentStatus {
    /// Pending
    Pending,
    /// Failed
    Failed,
    /// Successful
    Successful,
}

/// Fiat payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayment {
    /// Amount
    #[serde(rename = "Amount")]
    pub amount: PositiveDecimal,
    /// Fee amount
    #[serde(rename = "FeeAmount")]
    pub fee_amount: PositiveDecimal,
    /// Exchange rate
    #[serde(rename = "Rate")]
    pub rate: PositiveDecimal,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
    /// Fiat deposit ID (optional)
    #[serde(rename = "FiatDepositID")]
    pub fiat_deposit_id: Option<String>,
    /// Payment system ID (optional)
    #[serde(rename = "PaymentSystemID")]
    pub payment_system_id: Option<String>,
}

/// Transaction breakdown type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionBreakdownType {
    /// Channel fee
    ChannelFee,
    /// Business fee
    BusinessFee,
    /// Remaining amount
    Remaining,
}

/// Transaction breakdown item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionBreakdownItem {
    /// Breakdown type
    #[serde(rename = "Type")]
    pub breakdown_type: TransactionBreakdownType,
    /// Amount
    #[serde(rename = "Amount")]
    pub amount: PositiveDecimal,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    #[serde(rename = "ID")]
    pub id: String,
    /// Public ID (optional)
    #[serde(rename = "PublicID")]
    pub public_id: Option<String>,
    /// Network
    #[serde(rename = "Network")]
    pub network: Network,
    /// Created timestamp
    #[serde(rename = "Created")]
    pub created: DateTime,
    /// Status
    #[serde(rename = "Status")]
    pub status: TransactionStatus,
    /// Direction
    #[serde(rename = "Direction")]
    pub direction: TransactionDirection,
    /// Customer ID (optional)
    #[serde(rename = "CustomerID")]
    pub customer_id: Option<CustomerID>,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    pub external_id: Option<ExternalID>,
    /// Amount
    #[serde(rename = "Amount")]
    pub amount: PositiveDecimal,
    /// Network fee (optional)
    #[serde(rename = "NetworkFee")]
    pub network_fee: Option<PositiveDecimal>,
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Fiat payment (optional)
    #[serde(rename = "FiatPayment")]
    pub fiat_payment: Option<FiatPayment>,
    /// Orchestration (optional)
    #[serde(rename = "Orchestration")]
    pub orchestration: Option<serde_json::Value>,
    /// Fiat payment method (optional)
    #[serde(rename = "FiatPaymentMethod")]
    pub fiat_payment_method: Option<PaymentMethod>,
    /// Breakdown (optional)
    #[serde(rename = "Breakdown")]
    pub breakdown: Option<Vec<TransactionBreakdownItem>>,
}

/// Get transactions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    /// List of transactions
    #[serde(rename = "Items")]
    pub items: Vec<Transaction>,
    /// Pagination token
    #[serde(rename = "PageToken")]
    pub page_token: Option<String>,
}

/// Prepare sell request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareSellRequest {
    /// Channel ID
    #[serde(rename = "ChannelID")]
    pub channel_id: ChannelID,
    /// Payment method ID (optional)
    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: Option<PaymentMethodID>,
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Customer ID (optional)
    #[serde(rename = "CustomerID")]
    pub customer_id: Option<CustomerID>,
    /// Fiat amount
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: PositiveDecimal,
    /// Form (optional)
    #[serde(rename = "Form")]
    pub form: Option<serde_json::Value>,
    /// Delayed sell (optional)
    #[serde(rename = "DelayedSell")]
    pub delayed_sell: Option<bool>,
}

/// Prepare sell response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareSellResponse {
    /// Total fee
    #[serde(rename = "TotalFee")]
    pub total_fee: PositiveDecimal,
    /// Crypto amount estimate
    #[serde(rename = "CryptoAmountEstimate")]
    pub crypto_amount_estimate: PositiveDecimal,
    /// Crypto authorized amount
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: PositiveDecimal,
    /// Form session ID
    #[serde(rename = "FormSessionID")]
    pub form_session_id: FormSessionID,
}

/// Sell request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellRequest {
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Fiat amount
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: PositiveDecimal,
    /// Crypto authorized amount
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: PositiveDecimal,
    /// Form session ID
    #[serde(rename = "FormSessionID")]
    pub form_session_id: FormSessionID,
    /// Nonce
    #[serde(rename = "Nonce")]
    pub nonce: Nonce,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    pub external_id: Option<ExternalID>,
}

/// Sell response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellResponse {
    /// Transaction
    #[serde(rename = "Transaction")]
    pub transaction: Transaction,
}

