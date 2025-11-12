//! Checkout-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    /// Description
    #[serde(rename = "Description")]
    pub description: String,
    /// Quantity
    #[serde(rename = "Quantity")]
    pub quantity: PositiveDecimal,
    /// Unit amount
    #[serde(rename = "UnitAmount")]
    pub unit_amount: PositiveDecimal,
    /// Total amount
    #[serde(rename = "TotalAmount")]
    pub total_amount: PositiveDecimal,
}

/// Line items
pub type LineItems = Vec<LineItem>;

/// Checkout session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum CheckoutSessionStatus {
    /// Pending
    Pending,
    /// Failed
    Failed,
    /// Settled
    Settled,
}

/// Checkout session type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum CheckoutSessionType {
    /// Crypto payin
    PayinCrypto,
    /// Fiat payin
    PayinFiat,
    /// Fiat payout
    PayoutFiat,
}

/// Checkout session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    /// Checkout session ID
    #[serde(rename = "CheckoutSessionID")]
    pub checkout_session_id: String,
    /// Payment method category (optional)
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: Option<PaymentMethodCategory>,
    /// Source currency
    #[serde(rename = "SourceCurrency")]
    pub source_currency: String,
    /// Destination currency
    #[serde(rename = "DestinationCurrency")]
    pub destination_currency: String,
    /// Source amount
    #[serde(rename = "SourceAmount")]
    pub source_amount: PositiveDecimal,
    /// Destination amount
    #[serde(rename = "DestinationAmount")]
    pub destination_amount: PositiveDecimal,
    /// Authorized amount
    #[serde(rename = "AuthorizedAmount")]
    pub authorized_amount: Option<PositiveDecimal>,
    /// Status
    #[serde(rename = "Status")]
    pub status: String,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    pub external_id: Option<ExternalID>,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// Return URL
    #[serde(rename = "ReturnURL")]
    pub return_url: ReturnURL,
    /// Line items
    #[serde(rename = "LineItems")]
    pub line_items: LineItems,
    /// Type
    #[serde(rename = "Type")]
    pub session_type: String,
    /// Expiry (optional)
    #[serde(rename = "Expiry")]
    pub expiry: Option<DateTime>,
    /// Created timestamp
    #[serde(rename = "Created")]
    pub created: DateTime,
}

/// Checkout session response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSessionResponse {
    /// Hosted URL
    #[serde(rename = "HostedURL")]
    pub hosted_url: String,
    /// Checkout session
    #[serde(rename = "CheckoutSession")]
    pub checkout_session: CheckoutSession,
}

