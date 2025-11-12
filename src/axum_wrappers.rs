//! Axum wrapper types for deserialization and OpenAPI schema generation
//!
//! This module provides wrapper types that implement `Deserialize` for Axum's `Json` extractor
//! and `ToSchema` for OpenAPI documentation generation with `utoipa`.
//!
//! These types wrap the SDK's request and response types, allowing them to be used directly
//! with Axum's JSON extractor while maintaining compatibility with the SDK's internal types.
//!
//! # Example
//!
//! ```no_run
//! use axum::Json;
//! use noah_sdk::axum_wrappers::FiatPayoutRequestWrapper;
//!
//! async fn create_payout(
//!     Json(request): Json<FiatPayoutRequestWrapper>
//! ) -> Result<String, String> {
//!     let sdk_request: noah_sdk::api::checkout::FiatPayoutRequest = request.into();
//!     // Use sdk_request with the SDK client
//!     Ok("Success".to_string())
//! }
//! ```

#[cfg(feature = "axum")]
use crate::api::checkout::{CryptoPayinRequest, FiatPayinRequest, FiatPayoutRequest};
#[cfg(feature = "axum")]
use crate::api::onboarding::PrefillOnboardingRequest;
#[cfg(feature = "axum")]
use crate::models::balances::GetBalancesResponse;
#[cfg(feature = "axum")]
use crate::models::channels::{Channel, ChannelsCountriesResponse, GetChannelsResponse};
#[cfg(feature = "axum")]
use crate::models::checkout::{CheckoutSessionResponse, LineItem};
#[cfg(feature = "axum")]
use crate::models::common::*;
#[cfg(feature = "axum")]
use crate::models::customers::{Customer, CustomerInput, GetCustomersResponse};
#[cfg(feature = "axum")]
use crate::models::onboarding::{
    HostedOnboardingRequest, HostedSessionResponse, PrefillDocumentUploadURLResponse,
};
#[cfg(feature = "axum")]
use crate::models::payment_methods::GetPaymentMethodsResponse;
#[cfg(feature = "axum")]
use crate::models::transactions::{
    GetTransactionsResponse, PrepareSellRequest, PrepareSellResponse, SellRequest, SellResponse,
    Transaction,
};
#[cfg(feature = "axum")]
use crate::models::workflows::{
    BankDepositToOnchainAddressRequest, BankDepositToOnchainAddressResponse,
};
#[cfg(feature = "axum")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "axum")]
#[allow(unused_imports)]
use serde_json::json; // Used in #[schema(example = json!(...))] attributes
#[cfg(feature = "axum")]
use utoipa::ToSchema;

// ============================================================================
// Request Wrapper Types (for Deserialization)
// ============================================================================

/// Wrapper type that implements Deserialize for Axum Json extractor
/// This mirrors the SDK FiatPayoutRequest type but implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FiatPayoutRequestWrapper {
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,

    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,

    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,

    #[serde(rename = "ReturnURL")]
    pub return_url: String,

    #[serde(rename = "CustomerID")]
    pub customer_id: String,

    #[serde(rename = "ExternalID")]
    pub external_id: Option<String>,

    #[serde(rename = "Customer")]
    pub customer: Option<serde_json::Value>,

    #[serde(rename = "LineItems")]
    #[schema(value_type = Vec<Object>)]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "Nonce")]
    pub nonce: String,
}

#[cfg(feature = "axum")]
impl From<FiatPayoutRequestWrapper> for FiatPayoutRequest {
    fn from(wrapper: FiatPayoutRequestWrapper) -> Self {
        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc, // Default fallback
        };

        Self {
            crypto_currency,
            fiat_currency: wrapper.fiat_currency,
            fiat_amount: wrapper.fiat_amount,
            crypto_authorized_amount: wrapper.crypto_authorized_amount,
            return_url: wrapper.return_url,
            customer_id: wrapper.customer_id,
            external_id: wrapper.external_id,
            customer: wrapper.customer,
            line_items: wrapper.line_items,
            nonce: wrapper.nonce,
        }
    }
}

/// Wrapper type that implements Deserialize for Axum Json extractor
/// This mirrors the SDK CryptoPayinRequest type but implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CryptoPayinRequestWrapper {
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "CryptoAmount")]
    pub crypto_amount: String,

    #[serde(rename = "ReturnURL")]
    pub return_url: String,

    #[serde(rename = "CustomerID")]
    pub customer_id: String,

    #[serde(rename = "ExternalID")]
    pub external_id: Option<String>,

    #[serde(rename = "Customer")]
    pub customer: Option<serde_json::Value>,

    #[serde(rename = "LineItems")]
    #[schema(value_type = Vec<Object>)]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "Nonce")]
    pub nonce: String,
}

#[cfg(feature = "axum")]
impl From<CryptoPayinRequestWrapper> for CryptoPayinRequest {
    fn from(wrapper: CryptoPayinRequestWrapper) -> Self {
        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc, // Default fallback
        };

        Self {
            crypto_currency,
            crypto_amount: wrapper.crypto_amount,
            return_url: wrapper.return_url,
            customer_id: wrapper.customer_id,
            external_id: wrapper.external_id,
            customer: wrapper.customer,
            line_items: wrapper.line_items,
            nonce: wrapper.nonce,
        }
    }
}

/// Wrapper type that implements Deserialize for Axum Json extractor
/// This mirrors the SDK FiatPayinRequest type but implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FiatPayinRequestWrapper {
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: String,

    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,

    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,

    #[serde(rename = "ReturnURL")]
    pub return_url: String,

    #[serde(rename = "CustomerID")]
    pub customer_id: String,

    #[serde(rename = "ExternalID")]
    pub external_id: Option<String>,

    #[serde(rename = "Customer")]
    pub customer: Option<serde_json::Value>,

    #[serde(rename = "LineItems")]
    #[schema(value_type = Vec<Object>)]
    pub line_items: Vec<LineItem>,

    #[serde(rename = "Nonce")]
    pub nonce: String,
}

#[cfg(feature = "axum")]
impl From<FiatPayinRequestWrapper> for FiatPayinRequest {
    fn from(wrapper: FiatPayinRequestWrapper) -> Self {
        let payment_method_category = match wrapper.payment_method_category.as_str() {
            "Bank" => PaymentMethodCategory::Bank,
            "Card" => PaymentMethodCategory::Card,
            "Identifier" => PaymentMethodCategory::Identifier,
            _ => PaymentMethodCategory::Bank, // Default fallback
        };

        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc, // Default fallback
        };

        Self {
            payment_method_category,
            fiat_currency: wrapper.fiat_currency,
            crypto_currency,
            fiat_amount: wrapper.fiat_amount,
            return_url: wrapper.return_url,
            customer_id: wrapper.customer_id,
            external_id: wrapper.external_id,
            customer: wrapper.customer,
            line_items: wrapper.line_items,
            nonce: wrapper.nonce,
        }
    }
}

/// Wrapper type that implements Deserialize for Axum Json extractor
/// This mirrors the SDK PrefillOnboardingRequest type but implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PrefillOnboardingRequestWrapper {
    #[serde(rename = "Type")]
    pub request_type: Option<String>,

    #[serde(rename = "Token")]
    pub token: Option<String>,

    #[serde(flatten)]
    pub data: Option<serde_json::Value>,
}

#[cfg(feature = "axum")]
impl From<PrefillOnboardingRequestWrapper> for PrefillOnboardingRequest {
    fn from(wrapper: PrefillOnboardingRequestWrapper) -> Self {
        if let (Some(request_type), Some(token)) = (wrapper.request_type, wrapper.token) {
            PrefillOnboardingRequest::SumSubToken {
                request_type,
                token,
            }
        } else if let Some(data) = wrapper.data {
            // Try to determine if it's business or individual based on the data structure
            // For now, we'll use IndividualCustomerPrefill as default
            PrefillOnboardingRequest::IndividualCustomerPrefill(data)
        } else {
            // Default to a simple structure
            PrefillOnboardingRequest::IndividualCustomerPrefill(serde_json::json!({}))
        }
    }
}

// ============================================================================
// Query Parameter Types
// ============================================================================

/// Query parameters for list endpoints
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListQueryParams {
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}

/// Query parameters for get channels sell endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetChannelsSellQueryParams {
    pub country_code: Option<String>,
    pub crypto_currency: Option<String>,
    pub fiat_currency: Option<String>,
    pub fiat_amount: Option<String>,
    pub customer_id: Option<String>,
}

/// Query parameters for get channel endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetChannelQueryParams {
    pub crypto_currency: Option<String>,
    pub fiat_amount: Option<String>,
    pub customer_id: Option<String>,
}

/// Query parameters for get channels sell countries endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetChannelsSellCountriesQueryParams {
    pub customer_id: Option<String>,
}

/// Query parameters for get customers endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCustomersQueryParams {
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
    pub sort_direction: Option<String>,
}

/// Query parameters for get transactions endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetTransactionsQueryParams {
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
    pub sort_direction: Option<String>,
}

/// Query parameters for get payment methods endpoint
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetPaymentMethodsQueryParams {
    pub customer_id: String,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
}

// ============================================================================
// Response Wrapper Types (for OpenAPI Schema)
// ============================================================================

/// Wrapper for GetBalancesResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "items": [
            {
                "crypto_currency": "USDC",
                "total": "1000.00",
                "available": "950.00",
                "locked": "50.00"
            }
        ],
        "next_page_token": null
    })
)]
pub struct GetBalancesResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<GetBalancesResponse> for GetBalancesResponseWrapper {
    fn from(response: GetBalancesResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for GetChannelsResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "items": [
            {
                "id": "channel_123",
                "country": "US",
                "payment_method_category": "Bank",
                "fiat_currency": "USD",
                "min_fiat_amount": "10.00",
                "max_fiat_amount": "10000.00"
            }
        ],
        "next_page_token": null
    })
)]
pub struct GetChannelsResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<GetChannelsResponse> for GetChannelsResponseWrapper {
    fn from(response: GetChannelsResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for Channel
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "id": "channel_123",
        "country": "US",
        "payment_method_category": "Bank",
        "fiat_currency": "USD",
        "min_fiat_amount": "10.00",
        "max_fiat_amount": "10000.00"
    })
)]
pub struct ChannelWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Channel> for ChannelWrapper {
    fn from(response: Channel) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for ChannelsCountriesResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "countries": ["US", "GB", "CA"]
    })
)]
pub struct ChannelsCountriesResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<ChannelsCountriesResponse> for ChannelsCountriesResponseWrapper {
    fn from(response: ChannelsCountriesResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for CustomerInput
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "Individual": {
            "full_name": "John Doe",
            "date_of_birth": "1990-01-01",
            "email": "john.doe@example.com",
            "phone": "+1234567890"
        }
    })
)]
pub struct CustomerInputWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CustomerInput> for CustomerInputWrapper {
    fn from(input: CustomerInput) -> Self {
        Self {
            inner: serde_json::to_value(input).unwrap_or_default(),
        }
    }
}

#[cfg(feature = "axum")]
impl From<CustomerInputWrapper> for CustomerInput {
    fn from(wrapper: CustomerInputWrapper) -> Self {
        // Try to deserialize from JSON value
        // If it fails, try to create a minimal valid structure from the JSON
        match serde_json::from_value::<CustomerInput>(wrapper.inner.clone()) {
            Ok(input) => input,
            Err(_) => {
                // If deserialization fails, try to parse as IndividualCustomerInput directly
                match serde_json::from_value::<crate::models::customers::IndividualCustomerInput>(
                    wrapper.inner,
                ) {
                    Ok(individual) => CustomerInput::Individual(individual),
                    Err(_) => {
                        // Last resort: return an error by panicking
                        // This should never happen in practice as the JSON should be valid
                        panic!(
                            "Failed to deserialize CustomerInput from JSON. This indicates invalid input data."
                        );
                    }
                }
            }
        }
    }
}

/// Wrapper for Customer
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "Individual": {
            "id": "customer_123",
            "full_name": "John Doe",
            "date_of_birth": "1990-01-01",
            "email": "john.doe@example.com",
            "phone": "+1234567890",
            "status": "Active"
        }
    })
)]
pub struct CustomerWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Customer> for CustomerWrapper {
    fn from(response: Customer) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for GetCustomersResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "items": [
            {
                "Individual": {
                    "id": "customer_123",
                    "full_name": "John Doe",
                    "email": "john.doe@example.com"
                }
            }
        ],
        "next_page_token": null
    })
)]
pub struct GetCustomersResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<GetCustomersResponse> for GetCustomersResponseWrapper {
    fn from(response: GetCustomersResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for CheckoutSessionResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "checkout_session": {
            "id": "session_123",
            "url": "https://checkout.noah.com/session_123",
            "status": "pending"
        }
    })
)]
pub struct CheckoutSessionResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<CheckoutSessionResponse> for CheckoutSessionResponseWrapper {
    fn from(response: CheckoutSessionResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for HostedOnboardingRequest
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "return_url": "https://example.com/return",
        "fiat_options": ["USD", "EUR"],
        "form": null,
        "metadata": null
    })
)]
pub struct HostedOnboardingRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<HostedOnboardingRequest> for HostedOnboardingRequestWrapper {
    fn from(request: HostedOnboardingRequest) -> Self {
        Self {
            inner: serde_json::to_value(request).unwrap_or_default(),
        }
    }
}

#[cfg(feature = "axum")]
impl From<HostedOnboardingRequestWrapper> for HostedOnboardingRequest {
    fn from(wrapper: HostedOnboardingRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| HostedOnboardingRequest {
            return_url: String::new(),
            fiat_options: vec![],
            form: None,
            metadata: None,
        })
    }
}

/// Wrapper for HostedSessionResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "hosted_url": "https://onboarding.noah.com/session_123",
        "session_id": "session_123"
    })
)]
pub struct HostedSessionResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<HostedSessionResponse> for HostedSessionResponseWrapper {
    fn from(response: HostedSessionResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for PrefillDocumentUploadURLResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "upload_url": "https://upload.noah.com/document_123",
        "document_id": "doc_123"
    })
)]
pub struct PrefillDocumentUploadURLResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<PrefillDocumentUploadURLResponse> for PrefillDocumentUploadURLResponseWrapper {
    fn from(response: PrefillDocumentUploadURLResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for PrepareSellRequest
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "channel_id": "channel_123",
        "payment_method_id": "pm_123",
        "crypto_currency": "USDC",
        "customer_id": "customer_123",
        "fiat_amount": "100.00",
        "form": null,
        "delayed_sell": null
    })
)]
pub struct PrepareSellRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<PrepareSellRequest> for PrepareSellRequestWrapper {
    fn from(request: PrepareSellRequest) -> Self {
        Self {
            inner: serde_json::to_value(request).unwrap_or_default(),
        }
    }
}

#[cfg(feature = "axum")]
impl From<PrepareSellRequestWrapper> for PrepareSellRequest {
    fn from(wrapper: PrepareSellRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| PrepareSellRequest {
            channel_id: String::new(),
            payment_method_id: None,
            crypto_currency: CryptoCurrencyCode::Usdc,
            customer_id: None,
            fiat_amount: String::new(),
            form: None,
            delayed_sell: None,
        })
    }
}

/// Wrapper for PrepareSellResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "form_session_id": "form_session_123",
        "crypto_amount": "100.00",
        "fiat_amount": "100.00",
        "exchange_rate": "1.0"
    })
)]
pub struct PrepareSellResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<PrepareSellResponse> for PrepareSellResponseWrapper {
    fn from(response: PrepareSellResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for SellRequest
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "crypto_currency": "USDC",
        "fiat_amount": "100.00",
        "crypto_authorized_amount": "100.00",
        "form_session_id": "form_session_123",
        "nonce": "nonce_123",
        "external_id": "ext_123"
    })
)]
pub struct SellRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<SellRequest> for SellRequestWrapper {
    fn from(request: SellRequest) -> Self {
        Self {
            inner: serde_json::to_value(request).unwrap_or_default(),
        }
    }
}

#[cfg(feature = "axum")]
impl From<SellRequestWrapper> for SellRequest {
    fn from(wrapper: SellRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| SellRequest {
            crypto_currency: CryptoCurrencyCode::Usdc,
            fiat_amount: String::new(),
            crypto_authorized_amount: String::new(),
            form_session_id: String::new(),
            nonce: String::new(),
            external_id: None,
        })
    }
}

/// Wrapper for SellResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "transaction": {
            "id": "txn_123",
            "status": "pending",
            "direction": "sell",
            "crypto_currency": "USDC",
            "fiat_amount": "100.00",
            "crypto_amount": "100.00"
        }
    })
)]
pub struct SellResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<SellResponse> for SellResponseWrapper {
    fn from(response: SellResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for Transaction
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "id": "txn_123",
        "status": "completed",
        "direction": "sell",
        "crypto_currency": "USDC",
        "fiat_amount": "100.00",
        "crypto_amount": "100.00",
        "created_at": "2024-01-01T00:00:00Z"
    })
)]
pub struct TransactionWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<Transaction> for TransactionWrapper {
    fn from(response: Transaction) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for GetTransactionsResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "items": [
            {
                "id": "txn_123",
                "status": "completed",
                "direction": "sell",
                "crypto_currency": "USDC",
                "fiat_amount": "100.00"
            }
        ],
        "next_page_token": null
    })
)]
pub struct GetTransactionsResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<GetTransactionsResponse> for GetTransactionsResponseWrapper {
    fn from(response: GetTransactionsResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for GetPaymentMethodsResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "items": [
            {
                "id": "pm_123",
                "payment_method_category": "Bank",
                "status": "active",
                "fiat_currency": "USD"
            }
        ],
        "next_page_token": null
    })
)]
pub struct GetPaymentMethodsResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<GetPaymentMethodsResponse> for GetPaymentMethodsResponseWrapper {
    fn from(response: GetPaymentMethodsResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}

/// Wrapper for BankDepositToOnchainAddressRequest
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "customer_id": "customer_123",
        "fiat_currency": "USD",
        "crypto_currency": "USDC",
        "network": "Ethereum",
        "destination_address": {
            "address": "0x1234567890abcdef1234567890abcdef12345678"
        }
    })
)]
pub struct BankDepositToOnchainAddressRequestWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<BankDepositToOnchainAddressRequest> for BankDepositToOnchainAddressRequestWrapper {
    fn from(request: BankDepositToOnchainAddressRequest) -> Self {
        Self {
            inner: serde_json::to_value(request).unwrap_or_default(),
        }
    }
}

#[cfg(feature = "axum")]
impl From<BankDepositToOnchainAddressRequestWrapper> for BankDepositToOnchainAddressRequest {
    fn from(wrapper: BankDepositToOnchainAddressRequestWrapper) -> Self {
        serde_json::from_value(wrapper.inner).unwrap_or_else(|_| {
            BankDepositToOnchainAddressRequest {
                customer_id: String::new(),
                fiat_currency: String::new(),
                crypto_currency: CryptoCurrencyCode::Usdc,
                network: Network::Ethereum,
                destination_address: DestinationAddress {
                    address: String::new(),
                },
            }
        })
    }
}

/// Wrapper for BankDepositToOnchainAddressResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "payment_method_id": "pm_123",
        "workflow_id": "workflow_123",
        "status": "pending"
    })
)]
pub struct BankDepositToOnchainAddressResponseWrapper {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub inner: serde_json::Value,
}

#[cfg(feature = "axum")]
impl From<BankDepositToOnchainAddressResponse> for BankDepositToOnchainAddressResponseWrapper {
    fn from(response: BankDepositToOnchainAddressResponse) -> Self {
        Self {
            inner: serde_json::to_value(response).unwrap_or_default(),
        }
    }
}
