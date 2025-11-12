//! Channels API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::channels::{Channel, ChannelsCountriesResponse, GetChannelsResponse};
use crate::models::common::*;

impl NoahClient {
    /// Get channels for selling crypto (async)
    #[cfg(feature = "async")]
    pub async fn get_channels_sell(
        &self,
        crypto_currency: &CryptoCurrencyCode,
        country: Option<&CountryCode>,
        fiat_currency: Option<&FiatCurrencyCode>,
        fiat_amount: Option<&PositiveDecimal>,
        customer_id: Option<&CustomerID>,
        payment_method_id: Option<&PaymentMethodID>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetChannelsResponse> {
        let mut path = "/channels/sell".to_string();
        let mut query_params = vec![format!("CryptoCurrency={}", crypto_currency)];

        if let Some(c) = country {
            query_params.push(format!("Country={}", c));
        }
        if let Some(fc) = fiat_currency {
            query_params.push(format!("FiatCurrency={}", fc));
        }
        if let Some(fa) = fiat_amount {
            query_params.push(format!("FiatAmount={}", fa));
        }
        if let Some(cid) = customer_id {
            query_params.push(format!("CustomerID={}", cid));
        }
        if let Some(pmid) = payment_method_id {
            query_params.push(format!("PaymentMethodID={}", pmid));
        }
        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get(&path).await
    }

    /// Get channels for selling crypto (blocking)
    #[cfg(feature = "sync")]
    pub fn get_channels_sell_blocking(
        &self,
        crypto_currency: &CryptoCurrencyCode,
        country: Option<&CountryCode>,
        fiat_currency: Option<&FiatCurrencyCode>,
        fiat_amount: Option<&PositiveDecimal>,
        customer_id: Option<&CustomerID>,
        payment_method_id: Option<&PaymentMethodID>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetChannelsResponse> {
        let mut path = "/channels/sell".to_string();
        let mut query_params = vec![format!("CryptoCurrency={}", crypto_currency)];

        if let Some(c) = country {
            query_params.push(format!("Country={}", c));
        }
        if let Some(fc) = fiat_currency {
            query_params.push(format!("FiatCurrency={}", fc));
        }
        if let Some(fa) = fiat_amount {
            query_params.push(format!("FiatAmount={}", fa));
        }
        if let Some(cid) = customer_id {
            query_params.push(format!("CustomerID={}", cid));
        }
        if let Some(pmid) = payment_method_id {
            query_params.push(format!("PaymentMethodID={}", pmid));
        }
        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get_blocking(&path)
    }

    /// Get channel by ID (async)
    #[cfg(feature = "async")]
    pub async fn get_channel(
        &self,
        channel_id: &ChannelID,
        crypto_currency: &CryptoCurrencyCode,
        fiat_amount: Option<&PositiveDecimal>,
        customer_id: Option<&CustomerID>,
    ) -> Result<Channel> {
        let mut path = format!("/channels/{}", channel_id);
        let mut query_params = vec![format!("CryptoCurrency={}", crypto_currency)];

        if let Some(fa) = fiat_amount {
            query_params.push(format!("FiatAmount={}", fa));
        }
        if let Some(cid) = customer_id {
            query_params.push(format!("CustomerID={}", cid));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get(&path).await
    }

    /// Get channel by ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_channel_blocking(
        &self,
        channel_id: &ChannelID,
        crypto_currency: &CryptoCurrencyCode,
        fiat_amount: Option<&PositiveDecimal>,
        customer_id: Option<&CustomerID>,
    ) -> Result<Channel> {
        let mut path = format!("/channels/{}", channel_id);
        let mut query_params = vec![format!("CryptoCurrency={}", crypto_currency)];

        if let Some(fa) = fiat_amount {
            query_params.push(format!("FiatAmount={}", fa));
        }
        if let Some(cid) = customer_id {
            query_params.push(format!("CustomerID={}", cid));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get_blocking(&path)
    }

    /// Get countries for sell channels (async)
    #[cfg(feature = "async")]
    pub async fn get_channels_sell_countries(
        &self,
        customer_id: Option<&CustomerID>,
    ) -> Result<ChannelsCountriesResponse> {
        let mut path = "/channels/sell/countries".to_string();

        if let Some(cid) = customer_id {
            path.push_str(&format!("?CustomerID={}", cid));
        }

        self.get(&path).await
    }

    /// Get countries for sell channels (blocking)
    #[cfg(feature = "sync")]
    pub fn get_channels_sell_countries_blocking(
        &self,
        customer_id: Option<&CustomerID>,
    ) -> Result<ChannelsCountriesResponse> {
        let mut path = "/channels/sell/countries".to_string();

        if let Some(cid) = customer_id {
            path.push_str(&format!("?CustomerID={}", cid));
        }

        self.get_blocking(&path)
    }
}

