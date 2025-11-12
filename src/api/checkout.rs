//! Checkout API
//!
//! This module provides functionality to create checkout sessions for payments.
//! Supports crypto payin, fiat payin, and fiat payout operations.
//!
//! # Examples
//!
//! ## Create Crypto Payin Session
//!
//! ```no_run
//! use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
//! use noah_sdk::api::checkout::CryptoPayinRequest;
//! use noah_sdk::models::common::*;
//!
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Sandbox);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = NoahClient::new(config, auth)?;
//!
//! let request = CryptoPayinRequest {
//!     crypto_currency: CryptoCurrencyCode::Btc,
//!     crypto_amount: "0.001".to_string(),
//!     return_url: "https://example.com/return".to_string(),
//!     customer_id: "customer-123".to_string(),
//!     external_id: None,
//!     customer: None,
//!     line_items: Default::default(),
//!     nonce: "unique-nonce".to_string(),
//! };
//!
//! let session = client.create_crypto_payin_session(&request).await?;
//! println!("Checkout URL: {}", session.hosted_url);
//! # Ok(())
//! # }
//! ```

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::checkout::CheckoutSessionResponse;
use crate::models::common::*;
use serde::Serialize;

/// Request to create a cryptocurrency payin checkout session
///
/// This request is used to create a checkout session where customers can pay
/// using cryptocurrency (e.g., Bitcoin, Ethereum).
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::api::checkout::CryptoPayinRequest;
/// use noah_sdk::models::common::*;
///
/// let request = CryptoPayinRequest {
///     crypto_currency: CryptoCurrencyCode::Btc,
///     crypto_amount: "0.001".to_string(),
///     return_url: "https://example.com/return".to_string(),
///     customer_id: "customer-123".to_string(),
///     external_id: Some("order-456".to_string()),
///     customer: None,
///     line_items: Default::default(),
///     nonce: "unique-nonce-123".to_string(),
/// };
/// ```
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

/// Request to create a fiat currency payin checkout session
///
/// This request is used to create a checkout session where customers can pay
/// using fiat currency (e.g., USD, EUR) via various payment methods.
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::api::checkout::FiatPayinRequest;
/// use noah_sdk::models::common::*;
///
/// let request = FiatPayinRequest {
///     payment_method_category: PaymentMethodCategory::Card,
///     fiat_currency: "USD".to_string(),
///     crypto_currency: CryptoCurrencyCode::Btc,
///     fiat_amount: "100.00".to_string(),
///     return_url: "https://example.com/return".to_string(),
///     customer_id: "customer-123".to_string(),
///     external_id: None,
///     customer: None,
///     line_items: Default::default(),
///     nonce: "unique-nonce".to_string(),
/// };
/// ```
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

/// Request to create a fiat currency payout checkout session
///
/// This request is used to create a checkout session where customers can sell
/// cryptocurrency and receive fiat currency.
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::api::checkout::FiatPayoutRequest;
/// use noah_sdk::models::common::*;
///
/// let request = FiatPayoutRequest {
///     crypto_currency: CryptoCurrencyCode::Btc,
///     fiat_currency: "USD".to_string(),
///     fiat_amount: "1000.00".to_string(),
///     crypto_authorized_amount: "0.025".to_string(),
///     return_url: "https://example.com/return".to_string(),
///     customer_id: "customer-123".to_string(),
///     external_id: None,
///     customer: None,
///     line_items: Default::default(),
///     nonce: "unique-nonce".to_string(),
/// };
/// ```
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
    /// Create a cryptocurrency payin checkout session
    ///
    /// Creates a checkout session that allows customers to pay using cryptocurrency.
    /// Returns a session with a checkout URL that the customer can be redirected to.
    ///
    /// # Arguments
    ///
    /// * `request` - The crypto payin request containing payment details
    ///
    /// # Returns
    ///
    /// Returns a [`CheckoutSessionResponse`] containing the checkout URL and session details.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request data is invalid
    /// - The API request fails
    /// - Authentication fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::api::checkout::CryptoPayinRequest;
    /// use noah_sdk::models::common::*;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// let request = CryptoPayinRequest {
    ///     crypto_currency: CryptoCurrencyCode::Btc,
    ///     crypto_amount: "0.001".to_string(),
    ///     return_url: "https://example.com/return".to_string(),
    ///     customer_id: "customer-123".to_string(),
    ///     external_id: None,
    ///     customer: None,
    ///     line_items: Default::default(),
    ///     nonce: "unique-nonce".to_string(),
    /// };
    ///
    /// let session = client.create_crypto_payin_session(&request).await?;
    /// println!("Redirect customer to: {}", session.hosted_url);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_crypto_payin_session(
        &self,
        request: &CryptoPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payin/crypto", request).await
    }

    /// Create a cryptocurrency payin checkout session (blocking)
    ///
    /// Synchronous version of [`create_crypto_payin_session`](Self::create_crypto_payin_session).
    /// See that method for detailed documentation.
    #[cfg(feature = "sync")]
    pub fn create_crypto_payin_session_blocking(
        &self,
        request: &CryptoPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payin/crypto", request)
    }

    /// Create a fiat currency payin checkout session
    ///
    /// Creates a checkout session that allows customers to pay using fiat currency
    /// via various payment methods (e.g., credit card, bank transfer).
    ///
    /// # Arguments
    ///
    /// * `request` - The fiat payin request containing payment details
    ///
    /// # Returns
    ///
    /// Returns a [`CheckoutSessionResponse`] containing the checkout URL and session details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::api::checkout::FiatPayinRequest;
    /// use noah_sdk::models::common::*;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// let request = FiatPayinRequest {
    ///     payment_method_category: PaymentMethodCategory::Card,
    ///     fiat_currency: "USD".to_string(),
    ///     crypto_currency: CryptoCurrencyCode::Btc,
    ///     fiat_amount: "100.00".to_string(),
    ///     return_url: "https://example.com/return".to_string(),
    ///     customer_id: "customer-123".to_string(),
    ///     external_id: None,
    ///     customer: None,
    ///     line_items: Default::default(),
    ///     nonce: "unique-nonce".to_string(),
    /// };
    ///
    /// let session = client.create_fiat_payin_session(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_fiat_payin_session(
        &self,
        request: &FiatPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payin/fiat", request).await
    }

    /// Create a fiat currency payin checkout session (blocking)
    ///
    /// Synchronous version of [`create_fiat_payin_session`](Self::create_fiat_payin_session).
    /// See that method for detailed documentation.
    #[cfg(feature = "sync")]
    pub fn create_fiat_payin_session_blocking(
        &self,
        request: &FiatPayinRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payin/fiat", request)
    }

    /// Create a fiat currency payout checkout session
    ///
    /// Creates a checkout session that allows customers to sell cryptocurrency
    /// and receive fiat currency in return.
    ///
    /// # Arguments
    ///
    /// * `request` - The fiat payout request containing sale details
    ///
    /// # Returns
    ///
    /// Returns a [`CheckoutSessionResponse`] containing the checkout URL and session details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::api::checkout::FiatPayoutRequest;
    /// use noah_sdk::models::common::*;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// let request = FiatPayoutRequest {
    ///     crypto_currency: CryptoCurrencyCode::Btc,
    ///     fiat_currency: "USD".to_string(),
    ///     fiat_amount: "1000.00".to_string(),
    ///     crypto_authorized_amount: "0.025".to_string(),
    ///     return_url: "https://example.com/return".to_string(),
    ///     customer_id: "customer-123".to_string(),
    ///     external_id: None,
    ///     customer: None,
    ///     line_items: Default::default(),
    ///     nonce: "unique-nonce".to_string(),
    /// };
    ///
    /// let session = client.create_fiat_payout_session(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_fiat_payout_session(
        &self,
        request: &FiatPayoutRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post("/checkout/payout/fiat", request).await
    }

    /// Create a fiat currency payout checkout session (blocking)
    ///
    /// Synchronous version of [`create_fiat_payout_session`](Self::create_fiat_payout_session).
    /// See that method for detailed documentation.
    #[cfg(feature = "sync")]
    pub fn create_fiat_payout_session_blocking(
        &self,
        request: &FiatPayoutRequest,
    ) -> Result<CheckoutSessionResponse> {
        self.post_blocking("/checkout/payout/fiat", request)
    }
}
