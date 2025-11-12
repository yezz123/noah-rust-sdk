//! Checkout API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::checkout::CheckoutSessionResponse;
use crate::models::common::*;
use serde::Serialize;

/// Crypto payin request
#[derive(Debug, Clone, Serialize)]
pub struct CryptoPayinRequest {
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Crypto amount
    #[serde(rename = "CryptoAmount")]
    pub crypto_amount: PositiveDecimal,
    /// Return URL
    #[serde(rename = "ReturnURL")]
    pub return_url: ReturnURL,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<ExternalID>,
    /// Customer (optional)
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Line items
    #[serde(rename = "LineItems")]
    pub line_items: crate::models::checkout::LineItems,
    /// Nonce
    #[serde(rename = "Nonce")]
    pub nonce: Nonce,
}

/// Fiat payin request
#[derive(Debug, Clone, Serialize)]
pub struct FiatPayinRequest {
    /// Payment method category
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: PaymentMethodCategory,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Fiat amount
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: PositiveDecimal,
    /// Return URL
    #[serde(rename = "ReturnURL")]
    pub return_url: ReturnURL,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<ExternalID>,
    /// Customer (optional)
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Line items
    #[serde(rename = "LineItems")]
    pub line_items: crate::models::checkout::LineItems,
    /// Nonce
    #[serde(rename = "Nonce")]
    pub nonce: Nonce,
}

/// Fiat payout request
#[derive(Debug, Clone, Serialize)]
pub struct FiatPayoutRequest {
    /// Cryptocurrency
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: CryptoCurrencyCode,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
    /// Fiat amount
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: PositiveDecimal,
    /// Crypto authorized amount
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: PositiveDecimal,
    /// Return URL
    #[serde(rename = "ReturnURL")]
    pub return_url: ReturnURL,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// External ID (optional)
    #[serde(rename = "ExternalID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<ExternalID>,
    /// Customer (optional)
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Line items
    #[serde(rename = "LineItems")]
    pub line_items: crate::models::checkout::LineItems,
    /// Nonce
    #[serde(rename = "Nonce")]
    pub nonce: Nonce,
}

impl NoahClient {
    /// Create crypto payin session (async)
    #[cfg(feature = "async")]
    pub async fn create_crypto_payin_session(
        &self,
        request: &CryptoPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payin/crypto", request).await
    }

    /// Create crypto payin session (blocking)
    #[cfg(feature = "sync")]
    pub fn create_crypto_payin_session_blocking(
        &self,
        request: &CryptoPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payin/crypto", request)
    }

    /// Create fiat payin session (async)
    #[cfg(feature = "async")]
    pub async fn create_fiat_payin_session(
        &self,
        request: &FiatPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payin/fiat", request).await
    }

    /// Create fiat payin session (blocking)
    #[cfg(feature = "sync")]
    pub fn create_fiat_payin_session_blocking(
        &self,
        request: &FiatPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payin/fiat", request)
    }

    /// Create fiat payout session (async)
    #[cfg(feature = "async")]
    pub async fn create_fiat_payout_session(
        &self,
        request: &FiatPayoutRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payout/fiat", request).await
    }

    /// Create fiat payout session (blocking)
    #[cfg(feature = "sync")]
    pub fn create_fiat_payout_session_blocking(
        &self,
        request: &FiatPayoutRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payout/fiat", request)
    }
}

