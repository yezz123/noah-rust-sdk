//! Payment method-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Payment method display details (discriminated union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum PaymentMethodDisplayDetails {
    /// Bank display
    #[serde(rename = "FiatPaymentMethodBankDisplay")]
    Bank(FiatPaymentMethodBankDisplay),
    /// Card display
    #[serde(rename = "FiatPaymentMethodCardDisplay")]
    Card(FiatPaymentMethodCardDisplay),
    /// Identifier display
    #[serde(rename = "FiatPaymentMethodIdentifierDisplay")]
    Identifier(FiatPaymentMethodIdentifierDisplay),
}

/// Bank payment method display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPaymentMethodBankDisplay {
    /// Account number
    #[serde(rename = "AccountNumber")]
    pub account_number: Option<String>,
    /// Bank code
    #[serde(rename = "BankCode")]
    pub bank_code: Option<String>,
}

/// Card payment method display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPaymentMethodCardDisplay {
    /// Last 4 digits
    #[serde(rename = "Last4")]
    pub last4: String,
    /// Card scheme
    #[serde(rename = "Scheme")]
    pub scheme: FiatPaymentCardScheme,
}

/// Identifier payment method display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPaymentMethodIdentifierDisplay {
    /// Identifier type
    #[serde(rename = "IdentifierType")]
    pub identifier_type: PaymentMethodIdentifierType,
    /// Identifier value
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

/// Card scheme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum FiatPaymentCardScheme {
    /// Mastercard
    Mastercard,
    /// Visa
    Visa,
}

/// Payment method identifier type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentMethodIdentifierType {
    /// Phone number
    PhoneNumber,
    /// Email
    Email,
    /// Tax ID
    TaxId,
}

/// Payment method display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDisplay {
    /// Payment method ID
    #[serde(rename = "ID")]
    pub id: PaymentMethodID,
    /// Payment method type
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: PaymentMethodType,
    /// Display details
    #[serde(rename = "Details")]
    pub details: PaymentMethodDisplayDetails,
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Payment method ID
    #[serde(rename = "ID")]
    pub id: PaymentMethodID,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: Option<CustomerID>,
    /// Country
    #[serde(rename = "Country")]
    pub country: CountryCode,
    /// Payment method category
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: PaymentMethodCategory,
    /// Display details
    #[serde(rename = "DisplayDetails")]
    pub display_details: PaymentMethodDisplayDetails,
}

/// Get payment methods response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentMethodsResponse {
    /// List of payment methods
    #[serde(rename = "Items")]
    pub items: Vec<PaymentMethod>,
    /// Pagination token
    #[serde(rename = "PageToken")]
    pub page_token: Option<String>,
}

