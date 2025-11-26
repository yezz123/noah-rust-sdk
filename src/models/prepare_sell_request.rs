use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareSellRequest {
    /// Unique identifier for the channel.
    #[serde(rename = "ChannelID")]
    pub channel_id: uuid::Uuid,
    /// Existing payment method id to be used.
    #[serde(rename = "PaymentMethodID", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,
    /// Form input to be submitted based on returned FormSchema
    #[serde(rename = "Form", skip_serializing_if = "Option::is_none")]
    pub form: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// When enabled, balance checks are deferred until the final sell request, allowing the order to be prepared now and executed later.
    #[serde(rename = "DelayedSell", skip_serializing_if = "Option::is_none")]
    pub delayed_sell: Option<bool>,
}

impl PrepareSellRequest {
    pub fn new(
        channel_id: uuid::Uuid,
        crypto_currency: String,
        fiat_amount: String,
    ) -> PrepareSellRequest {
        PrepareSellRequest {
            channel_id,
            payment_method_id: None,
            crypto_currency,
            customer_id: None,
            fiat_amount,
            form: None,
            delayed_sell: None,
        }
    }
}
