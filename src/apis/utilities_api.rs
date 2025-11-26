use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`balances_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BalancesGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`channels_channel_id_form_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelsChannelIdFormGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`channels_channel_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelsChannelIdGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`channels_sell_countries_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelsSellCountriesGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`channels_sell_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelsSellGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`customers_customer_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomersCustomerIdGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`customers_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomersGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`payment_methods_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentMethodsGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prices_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PricesGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_transaction_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsTransactionIdGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// This endpoint retrieves a paginated list of balances for the Business User's account.
pub async fn balances_get(
    configuration: &configuration::Configuration,
    page_size: Option<i32>,
    page_token: Option<&str>,
    api_signature: Option<&str>,
) -> Result<models::GetBalancesResponse, Error<BalancesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page_size = page_size;
    let p_query_page_token = page_token;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/balances", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBalancesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBalancesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BalancesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint provides a [JSONForm](https://jsonforms.io/) schema needed to configure payment methods for transactions on a specified channel. Use this schema to dynamically generate forms based on the selected payment method. The `ChannelID` parameter, obtainable from the Supported Channels endpoint, specifies the target channel for which the form is generated.  Follow the step-by-step guides:  * [Direct Payout to US Business](../recipes/payout/global-payouts-business)  * [Direct Payout to Individual Customer](../recipes/payout/global-payouts-individual)  **Note:** It is not possible to fetch schemas for channels where the `PaymentMethodType` is set to `TokenizedCard`, as the API does not directly accept credit card details. For credit card payments, refer to the [Hosted Checkout](../recipes/payout/hosted-checkout) solution.
pub async fn channels_channel_id_form_get(
    configuration: &configuration::Configuration,
    channel_id: &str,
    customer_id: Option<&str>,
    payment_method_id: Option<&str>,
    api_signature: Option<&str>,
) -> Result<models::GetFormResponse, Error<ChannelsChannelIdFormGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_customer_id = customer_id;
    let p_query_payment_method_id = payment_method_id;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/channels/{ChannelID}/form",
        configuration.base_path,
        ChannelID = crate::apis::urlencode(p_path_channel_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_customer_id {
        req_builder = req_builder.query(&[("CustomerID", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_payment_method_id {
        req_builder = req_builder.query(&[("PaymentMethodID", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFormResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFormResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChannelsChannelIdFormGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves detailed information about a specific channel by its ID.  The endpoint returns a single channel with calculated fees, limits, and processing information for the specified cryptocurrency and optional fiat amount.  Use this endpoint when you know the specific channel ID and want to get detailed information about that channel.
pub async fn channels_channel_id_get(
    configuration: &configuration::Configuration,
    channel_id: &str,
    crypto_currency: &str,
    fiat_amount: Option<&str>,
    customer_id: Option<&str>,
    api_signature: Option<&str>,
) -> Result<models::Channel, Error<ChannelsChannelIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_crypto_currency = crypto_currency;
    let p_query_fiat_amount = fiat_amount;
    let p_query_customer_id = customer_id;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/channels/{ChannelID}",
        configuration.base_path,
        ChannelID = crate::apis::urlencode(p_path_channel_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("CryptoCurrency", &p_query_crypto_currency.to_string())]);
    if let Some(ref param_value) = p_query_fiat_amount {
        req_builder = req_builder.query(&[("FiatAmount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_customer_id {
        req_builder = req_builder.query(&[("CustomerID", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Channel`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Channel`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChannelsChannelIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves the complete list of countries where Sell operations are supported.  The response is useful for enabling customers to select their desired Country+FiatCurrency combination for receiving payouts.  After making a selection, obtain the channels for the chosen Country+FiatCurrency using `GET /channels/sell`.  Follow the step-by-step guides:  * [Direct Payout to US Business](../recipes/payout/global-payouts-business)  * [Direct Payout to Individual Customer](../recipes/payout/global-payouts-individual)
pub async fn channels_sell_countries_get(
    configuration: &configuration::Configuration,
    customer_id: Option<&str>,
    api_signature: Option<&str>,
) -> Result<std::collections::HashMap<String, Vec<String>>, Error<ChannelsSellCountriesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_customer_id = customer_id;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/channels/sell/countries", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_customer_id {
        req_builder = req_builder.query(&[("CustomerID", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `std::collections::HashMap&lt;String, Vec&lt;String&gt;&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `std::collections::HashMap&lt;String, Vec&lt;String&gt;&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChannelsSellCountriesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint provides the list of supported channels for selling crypto into fiat.  Follow the step-by-step guides:  * [Direct Payout to US Business](../recipes/payout/global-payouts-business)  * [Direct Payout to Individual Customer](../recipes/payout/global-payouts-individual)
pub async fn channels_sell_get(
    configuration: &configuration::Configuration,
    crypto_currency: &str,
    country: Option<&str>,
    fiat_currency: Option<&str>,
    fiat_amount: Option<&str>,
    customer_id: Option<&str>,
    payment_method_id: Option<&str>,
    page_size: Option<i32>,
    page_token: Option<&str>,
    api_signature: Option<&str>,
) -> Result<models::GetChannelsResponse, Error<ChannelsSellGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_crypto_currency = crypto_currency;
    let p_query_country = country;
    let p_query_fiat_currency = fiat_currency;
    let p_query_fiat_amount = fiat_amount;
    let p_query_customer_id = customer_id;
    let p_query_payment_method_id = payment_method_id;
    let p_query_page_size = page_size;
    let p_query_page_token = page_token;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/channels/sell", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("Country", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("CryptoCurrency", &p_query_crypto_currency.to_string())]);
    if let Some(ref param_value) = p_query_fiat_currency {
        req_builder = req_builder.query(&[("FiatCurrency", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fiat_amount {
        req_builder = req_builder.query(&[("FiatAmount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_customer_id {
        req_builder = req_builder.query(&[("CustomerID", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_payment_method_id {
        req_builder = req_builder.query(&[("PaymentMethodID", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetChannelsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetChannelsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ChannelsSellGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves the details of a specific customer by their `CustomerID`.
pub async fn customers_customer_id_get(
    configuration: &configuration::Configuration,
    customer_id: &str,
) -> Result<models::Customer, Error<CustomersCustomerIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_customer_id = customer_id;

    let uri_str = format!(
        "{}/customers/{CustomerID}",
        configuration.base_path,
        CustomerID = crate::apis::urlencode(p_path_customer_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Customer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Customer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CustomersCustomerIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves a paginated list of customers for the Business User's account.
pub async fn customers_get(
    configuration: &configuration::Configuration,
    page_size: Option<i32>,
    page_token: Option<&str>,
    sort_direction: Option<models::SortDirection>,
    api_signature: Option<&str>,
) -> Result<models::GetCustomersResponse, Error<CustomersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page_size = page_size;
    let p_query_page_token = page_token;
    let p_query_sort_direction = sort_direction;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/customers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_direction {
        req_builder = req_builder.query(&[("SortDirection", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCustomersResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCustomersResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CustomersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves a paginated list of payment methods for a specific customer.
pub async fn payment_methods_get(
    configuration: &configuration::Configuration,
    customer_id: &str,
    page_size: Option<i32>,
    page_token: Option<&str>,
    api_signature: Option<&str>,
) -> Result<models::GetPaymentMethodsResponse, Error<PaymentMethodsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_customer_id = customer_id;
    let p_query_page_size = page_size;
    let p_query_page_token = page_token;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/payment-methods", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("CustomerID", &p_query_customer_id.to_string())]);
    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetPaymentMethodsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetPaymentMethodsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PaymentMethodsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint lets you retrieve real-time information about a conversion between two supported currencies, including the rate before and after the Noah Fee is applied.  During a Noah Hosted Checkout session, real-time `/prices` data is used to calculate the fees and thus the final amount a customer will pay. If neither SourceAmount or DestinationAmount is defined, the fee is not available.
pub async fn prices_get(
    configuration: &configuration::Configuration,
    source_currency: &str,
    destination_currency: &str,
    source_amount: Option<&str>,
    destination_amount: Option<&str>,
    payment_method_category: Option<&str>,
    country: Option<&str>,
) -> Result<models::GetPricesResponse, Error<PricesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_source_currency = source_currency;
    let p_query_destination_currency = destination_currency;
    let p_query_source_amount = source_amount;
    let p_query_destination_amount = destination_amount;
    let p_query_payment_method_category = payment_method_category;
    let p_query_country = country;

    let uri_str = format!("{}/prices", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("SourceCurrency", &p_query_source_currency.to_string())]);
    req_builder = req_builder.query(&[(
        "DestinationCurrency",
        &p_query_destination_currency.to_string(),
    )]);
    if let Some(ref param_value) = p_query_source_amount {
        req_builder = req_builder.query(&[("SourceAmount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_destination_amount {
        req_builder = req_builder.query(&[("DestinationAmount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_payment_method_category {
        req_builder = req_builder.query(&[("PaymentMethodCategory", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("Country", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetPricesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetPricesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PricesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves a paginated list of transaction history for the Business User's account.
pub async fn transactions_get(
    configuration: &configuration::Configuration,
    page_size: Option<i32>,
    page_token: Option<&str>,
    sort_direction: Option<models::SortDirection>,
    api_signature: Option<&str>,
) -> Result<models::GetTransactionsResponse, Error<TransactionsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page_size = page_size;
    let p_query_page_token = page_token;
    let p_query_sort_direction = sort_direction;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/transactions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("PageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_token {
        req_builder = req_builder.query(&[("PageToken", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_direction {
        req_builder = req_builder.query(&[("SortDirection", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTransactionsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTransactionsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TransactionsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint retrieves the details of a single transaction by `TransactionID`.
pub async fn transactions_transaction_id_get(
    configuration: &configuration::Configuration,
    transaction_id: &str,
    api_signature: Option<&str>,
) -> Result<models::Transaction, Error<TransactionsTransactionIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transaction_id = transaction_id;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/transactions/{TransactionID}",
        configuration.base_path,
        TransactionID = crate::apis::urlencode(p_path_transaction_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_api_signature {
        req_builder = req_builder.header("Api-Signature", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Transaction`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Transaction`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TransactionsTransactionIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
