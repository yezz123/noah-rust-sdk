//! Channel-related models

use crate::models::common::*;
use crate::models::payment_methods::PaymentMethodDisplay;
use serde::{Deserialize, Serialize};

/// Channel limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelLimits {
    /// Minimum limit
    #[serde(rename = "MinLimit")]
    pub min_limit: PositiveDecimal,
    /// Maximum limit
    #[serde(rename = "MaxLimit")]
    pub max_limit: Option<PositiveDecimal>,
}

/// Channel calculated fees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCalculated {
    /// Total fee
    #[serde(rename = "TotalFee")]
    pub total_fee: PositiveDecimal,
}

/// Channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel ID
    #[serde(rename = "ID")]
    pub id: ChannelID,
    /// Payment method category
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: PaymentMethodCategory,
    /// Payment method type
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: PaymentMethodType,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
    /// Country
    #[serde(rename = "Country")]
    pub country: CountryCode,
    /// Calculated fees (optional)
    #[serde(rename = "Calculated")]
    pub calculated: Option<ChannelCalculated>,
    /// Limits
    #[serde(rename = "Limits")]
    pub limits: ChannelLimits,
    /// Exchange rate
    #[serde(rename = "Rate")]
    pub rate: PositiveDecimal,
    /// Payment methods (optional)
    #[serde(rename = "PaymentMethods")]
    pub payment_methods: Option<Vec<PaymentMethodDisplay>>,
    /// Processing time in seconds
    #[serde(rename = "ProcessingSeconds")]
    pub processing_seconds: u64,
    /// Processing tier
    #[serde(rename = "ProcessingTier")]
    pub processing_tier: Option<ProcessingTier>,
    /// Form schema (optional)
    #[serde(rename = "FormSchema")]
    pub form_schema: Option<serde_json::Value>,
}

/// Get channels response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelsResponse {
    /// List of channels
    #[serde(rename = "Items")]
    pub items: Vec<Channel>,
    /// Pagination token
    #[serde(rename = "PageToken")]
    pub page_token: Option<String>,
}

/// Channels countries response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsCountriesResponse {
    #[serde(flatten)]
    pub countries: std::collections::HashMap<CountryCode, Vec<FiatCurrencyCode>>,
}

