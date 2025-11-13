//! Axum wrapper types for deserialization and OpenAPI schema generation
//!
//! This module provides wrapper types that implement `Deserialize` for Axum's `Json` extractor
//! and `ToSchema` for OpenAPI documentation generation with `utoipa`.
//!
//! These types wrap the SDK's request types, allowing them to be used directly
//! with Axum's JSON extractor while maintaining compatibility with the SDK's internal types.

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
use utoipa::ToSchema;

// ============================================================================
// Request Wrapper Types (for Deserialization)
// ============================================================================

/// Wrapper for FiatPayoutRequest that implements Deserialize
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
            _ => CryptoCurrencyCode::Usdc,
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

/// Wrapper for CryptoPayinRequest that implements Deserialize
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
            _ => CryptoCurrencyCode::Usdc,
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

/// Wrapper for FiatPayinRequest that implements Deserialize
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
            _ => PaymentMethodCategory::Bank,
        };

        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc,
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

/// Wrapper for PrefillOnboardingRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PrefillOnboardingRequestWrapper {
    #[serde(rename = "Type")]
    pub request_type: Option<String>,

    #[serde(rename = "Token")]
    pub token: Option<String>,

    // Note: flatten doesn't work well with ToSchema derive, so we'll accept it as a regular field
    // and handle the conversion in the From impl
    #[serde(flatten)]
    #[schema(ignore)]
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
            PrefillOnboardingRequest::IndividualCustomerPrefill(data)
        } else {
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
// Additional Request Wrapper Types
// ============================================================================

/// Wrapper for PrepareSellRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PrepareSellRequestWrapper {
    #[serde(rename = "ChannelID")]
    pub channel_id: String,

    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: Option<String>,

    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "CustomerID")]
    pub customer_id: Option<String>,

    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,

    #[serde(rename = "Form")]
    pub form: Option<serde_json::Value>,

    #[serde(rename = "DelayedSell")]
    pub delayed_sell: Option<bool>,
}

#[cfg(feature = "axum")]
impl From<PrepareSellRequestWrapper> for PrepareSellRequest {
    fn from(wrapper: PrepareSellRequestWrapper) -> Self {
        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc,
        };

        Self {
            channel_id: wrapper.channel_id,
            payment_method_id: wrapper.payment_method_id,
            crypto_currency,
            customer_id: wrapper.customer_id,
            fiat_amount: wrapper.fiat_amount,
            form: wrapper.form,
            delayed_sell: wrapper.delayed_sell,
        }
    }
}

/// Wrapper for SellRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SellRequestWrapper {
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,

    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,

    #[serde(rename = "FormSessionID")]
    pub form_session_id: String,

    #[serde(rename = "Nonce")]
    pub nonce: String,

    #[serde(rename = "ExternalID")]
    pub external_id: Option<String>,
}

#[cfg(feature = "axum")]
impl From<SellRequestWrapper> for SellRequest {
    fn from(wrapper: SellRequestWrapper) -> Self {
        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc,
        };

        Self {
            crypto_currency,
            fiat_amount: wrapper.fiat_amount,
            crypto_authorized_amount: wrapper.crypto_authorized_amount,
            form_session_id: wrapper.form_session_id,
            nonce: wrapper.nonce,
            external_id: wrapper.external_id,
        }
    }
}

/// Wrapper for HostedOnboardingRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HostedOnboardingRequestWrapper {
    #[serde(rename = "ReturnURL")]
    pub return_url: String,

    #[serde(rename = "FiatOptions")]
    pub fiat_options: Vec<FiatOptionWrapper>,

    #[serde(rename = "Form")]
    pub form: Option<serde_json::Value>,

    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FiatOptionWrapper {
    #[serde(rename = "FiatCurrencyCode")]
    pub fiat_currency_code: String,
}

#[cfg(feature = "axum")]
impl From<HostedOnboardingRequestWrapper> for HostedOnboardingRequest {
    fn from(wrapper: HostedOnboardingRequestWrapper) -> Self {
        use crate::models::onboarding::FiatOption;
        Self {
            return_url: wrapper.return_url,
            fiat_options: wrapper
                .fiat_options
                .into_iter()
                .map(|opt| FiatOption {
                    fiat_currency_code: opt.fiat_currency_code,
                })
                .collect(),
            form: wrapper.form,
            metadata: wrapper.metadata,
        }
    }
}

/// Wrapper for BankDepositToOnchainAddressRequest that implements Deserialize
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BankDepositToOnchainAddressRequestWrapper {
    #[serde(rename = "CustomerID")]
    pub customer_id: String,

    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,

    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,

    #[serde(rename = "Network")]
    pub network: String,

    #[serde(rename = "DestinationAddress")]
    pub destination_address: DestinationAddressWrapper,
}

#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DestinationAddressWrapper {
    #[serde(rename = "Address")]
    pub address: String,
}

#[cfg(feature = "axum")]
impl From<BankDepositToOnchainAddressRequestWrapper> for BankDepositToOnchainAddressRequest {
    fn from(wrapper: BankDepositToOnchainAddressRequestWrapper) -> Self {
        let crypto_currency = match wrapper.crypto_currency.as_str() {
            "BTC" => CryptoCurrencyCode::Btc,
            "BTC_TEST" => CryptoCurrencyCode::BtcTest,
            "USDC" => CryptoCurrencyCode::Usdc,
            "USDC_TEST" => CryptoCurrencyCode::UsdcTest,
            _ => CryptoCurrencyCode::Usdc,
        };

        let network = match wrapper.network.as_str() {
            "Bitcoin" => Network::Bitcoin,
            "BitcoinTest" => Network::BitcoinTest,
            "Ethereum" => Network::Ethereum,
            "EthereumTestSepolia" => Network::EthereumTestSepolia,
            "Celo" => Network::Celo,
            "CeloTestSepolia" => Network::CeloTestSepolia,
            "FlowEvm" => Network::FlowEvm,
            "FlowEvmTest" => Network::FlowEvmTest,
            "Gnosis" => Network::Gnosis,
            "GnosisTestChiado" => Network::GnosisTestChiado,
            "Lightning" => Network::Lightning,
            "LightningTest" => Network::LightningTest,
            "PolygonPos" => Network::PolygonPos,
            "PolygonTestAmoy" => Network::PolygonTestAmoy,
            "Solana" => Network::Solana,
            "SolanaDevnet" => Network::SolanaDevnet,
            "OffNetwork" => Network::OffNetwork,
            _ => Network::Ethereum,
        };

        Self {
            customer_id: wrapper.customer_id,
            fiat_currency: wrapper.fiat_currency,
            crypto_currency,
            network,
            destination_address: DestinationAddress {
                address: wrapper.destination_address.address,
            },
        }
    }
}

// ============================================================================
// Response Wrapper Types (for OpenAPI Schema)
// ============================================================================

/// Wrapper for GetBalancesResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
        match serde_json::from_value::<CustomerInput>(wrapper.inner.clone()) {
            Ok(input) => input,
            Err(_) => {
                match serde_json::from_value::<crate::models::customers::IndividualCustomerInput>(
                    wrapper.inner,
                ) {
                    Ok(individual) => CustomerInput::Individual(individual),
                    Err(_) => {
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

/// Wrapper for HostedSessionResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

/// Wrapper for PrepareSellResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

/// Wrapper for SellResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

/// Wrapper for BankDepositToOnchainAddressResponse
#[cfg(feature = "axum")]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
