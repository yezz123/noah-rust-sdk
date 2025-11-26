use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`checkout_payout_fiat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckoutPayoutFiatPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_sell_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsSellPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_sell_prepare_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsSellPreparePostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`workflows_onchain_deposit_to_payment_method_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowsOnchainDepositToPaymentMethodPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// This endpoint initiates Noah's Hosted Fiat Payout Session flow.  The solution establishes an end-to-end checkout experience where customers convert cryptocurrency to fiat through a hosted interface, with Noah handling KYC verification, payment method selection, and transaction monitoring.  Use the endpoint to retrieve a response consisting of a URL, which you pass to your customer so that they can complete their payout through a hosted session. If customer details are filled out, a new customer will be created. Otherwise, the provided `CustomerID` should refer to an existing customer.  Follow the step-by-step guide: [Hosted Fiat Payout](../recipes/payout/hosted-checkout)
pub async fn checkout_payout_fiat_post(
    configuration: &configuration::Configuration,
    checkout_payout_fiat_post_request: models::CheckoutPayoutFiatPostRequest,
    api_signature: Option<&str>,
) -> Result<models::CheckoutSessionResponse, Error<CheckoutPayoutFiatPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_checkout_payout_fiat_post_request = checkout_payout_fiat_post_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/checkout/payout/fiat", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
    req_builder = req_builder.json(&p_body_checkout_payout_fiat_post_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckoutSessionResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckoutSessionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckoutPayoutFiatPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint initiates an immediate transaction to sell specified cryptocurrency and send the resulting fiat currency to a designated payout method.  Use this endpoint to convert a crypto balance into fiat and send it to a beneficiary (e.g., bank transfer, card payout, wallet) with real-time execution, dynamic payout-form submission for payment methods, immediate balance updates, and end-to-end transaction tracking.  **Note:** This endpoint is only available to customers created under the Reliance Model. Before you get started with this endpoint, Noah must have authorized your usage of the Reliance Model. For more details on this process, see the [Compliance Overview](../getting-started/kyc). When using the Standard Model, use the [Onchain Deposit to Fiat Payout](onchain-deposit-to-fiat-payout) endpoint.  Follow the step-by-step guides:  * [Direct Payout to US Business](../recipes/payout/global-payouts-business)  * [Direct Payout to Individual Customer](../recipes/payout/global-payouts-individual)
pub async fn transactions_sell_post(
    configuration: &configuration::Configuration,
    sell_request: models::SellRequest,
    api_signature: Option<&str>,
) -> Result<models::SellResponse, Error<TransactionsSellPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_sell_request = sell_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/transactions/sell", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
    req_builder = req_builder.json(&p_body_sell_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SellResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SellResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TransactionsSellPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint calculates and validates the required inputs for a sell transaction, including accurate pricing estimates, fees, and limits.  Use the endpoint to pre-validate a crypto-to-fiat sell by generating a FormSessionID, retrieving real-time price/fee/limit estimates, computing the required CryptoAuthorizedAmount, and validating payout-form inputs.  This enables the subsequent Create Sell call to execute deterministically within your slippage, balance, and compliance constraints, with no funds being removed at this stage.  Follow the step-by-step guides:  * [Direct Payout to US Business](../recipes/payout/global-payouts-business)  * [Direct Payout to Individual Customer](../recipes/payout/global-payouts-individual)
pub async fn transactions_sell_prepare_post(
    configuration: &configuration::Configuration,
    prepare_sell_request: models::PrepareSellRequest,
    api_signature: Option<&str>,
) -> Result<models::PrepareSellResponse, Error<TransactionsSellPreparePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_prepare_sell_request = prepare_sell_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/transactions/sell/prepare", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
    req_builder = req_builder.json(&p_body_prepare_sell_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PrepareSellResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PrepareSellResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TransactionsSellPreparePostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint establishes an automated workflow that converts incoming cryptocurrency deposits into fiat currency payouts.  The solution requires configuration of source cryptocurrency, a FormSessionID for payout instructions, recipient payment method details, and conversion parameters, with deposited amounts traded for the specified fiat currency at current market prices after application of fees.  The customer must already exist before using this endpoint.   **Notes**:   * On-chain operations are irreversible, and market prices may fluctuate significantly, impacting the final payout amount.  * Although this workflow should continue working with the original FormSession, Noah cannot guarantee FormSession longevity. For this reason, generate a new FormSession each time you present the customer with the onchain deposit address.  * If the customer reuses the onchain address after the FormSession becomes invalid, the transfer will fail but funds will not be lost.
pub async fn workflows_onchain_deposit_to_payment_method_post(
    configuration: &configuration::Configuration,
    onchain_deposit_to_payment_method_request: models::OnchainDepositToPaymentMethodRequest,
    api_signature: Option<&str>,
) -> Result<
    models::OnchainDepositToPaymentMethodResponse,
    Error<WorkflowsOnchainDepositToPaymentMethodPostError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_onchain_deposit_to_payment_method_request =
        onchain_deposit_to_payment_method_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/workflows/onchain-deposit-to-payment-method",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
    req_builder = req_builder.json(&p_body_onchain_deposit_to_payment_method_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OnchainDepositToPaymentMethodResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OnchainDepositToPaymentMethodResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<WorkflowsOnchainDepositToPaymentMethodPostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
