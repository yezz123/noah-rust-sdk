use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`checkout_payin_crypto_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckoutPayinCryptoPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`checkout_payin_fiat_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckoutPayinFiatPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`hosted_workflows_bank_deposit_to_onchain_address_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HostedWorkflowsBankDepositToOnchainAddressPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sandbox_fiat_deposit_simulate_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SandboxFiatDepositSimulatePostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`workflows_bank_deposit_to_onchain_address_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowsBankDepositToOnchainAddressPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// This endpoint initiates Noah’s hosted checkout session for cryptocurrency payments.  The solution creates a hosted session where customers make cryptocurrency payments through Noah’s hosted checkout page, requiring properties such as a `CustomerID` for customer identification, CryptoCurrency type (like USDC or Bitcoin), Amount specifications, and return/cancel URLs for post-payment redirection.  Use the endpoint to retrieve a response consisting of a checkout URL, which you pass to your customer so that they can complete their cryptocurrency payment through a hosted session.  The hosted flow handles the complete payment process with real-time status updates via Webhooks.
pub async fn checkout_payin_crypto_post(
    configuration: &configuration::Configuration,
    checkout_payin_crypto_post_request: models::CheckoutPayinCryptoPostRequest,
    api_signature: Option<&str>,
) -> Result<models::CheckoutSessionResponse, Error<CheckoutPayinCryptoPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_checkout_payin_crypto_post_request = checkout_payin_crypto_post_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/checkout/payin/crypto", configuration.base_path);
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
    req_builder = req_builder.json(&p_body_checkout_payin_crypto_post_request);

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
        let entity: Option<CheckoutPayinCryptoPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint initiates Noah’s hosted checkout session for fiat currency payments.  The solution creates a hosted session where customers make fiat currency payments through Noah’s hosted checkout interface, accepting parameters, including customer details. If customer details are filled out, a new customer will be created. Otherwise, the provided `CustomerID` should refer to an existing customer.  Use the endpoint to retrieve a response consisting of a checkout URL, which you pass to your customer so that they can complete their fiat payment through a hosted session.  The hosted flow handles the complete payment process with real-time status updates via Webhooks.
pub async fn checkout_payin_fiat_post(
    configuration: &configuration::Configuration,
    checkout_payin_fiat_post_request: models::CheckoutPayinFiatPostRequest,
    api_signature: Option<&str>,
) -> Result<models::CheckoutSessionResponse, Error<CheckoutPayinFiatPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_checkout_payin_fiat_post_request = checkout_payin_fiat_post_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/checkout/payin/fiat", configuration.base_path);
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
    req_builder = req_builder.json(&p_body_checkout_payin_fiat_post_request);

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
        let entity: Option<CheckoutPayinFiatPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint initiates Noah’s hosted onboarding session for automated fiat-to-crypto workflows.  The solution creates a hosted session where customers can set up a workflow that automatically converts incoming fiat currency from bank deposits into cryptocurrency and sends the acquired crypto to a specified wallet address on the specified network. The workflow triggers automatically and indefinitely whenever a bank transfer is made to the designated bank account.  Use the endpoint to retrieve a response consisting of a URL, which you pass to your customer so that they can configure their automated conversion workflow through a hosted session. Deposited amounts are traded for the specified cryptocurrency at current market prices after application of fees.  **Notes**:   * On-chain operations are irreversible, and market prices may fluctuate significantly, impacting the final payout amount.  * Although this workflow should continue working with the original FormSession, Noah cannot guarantee FormSession longevity. For this reason, generate a new FormSession each time you present the customer with the onchain deposit address.  * If the customer reuses the onchain address after the FormSession becomes invalid, the transfer will fail but funds will not be lost.
pub async fn hosted_workflows_bank_deposit_to_onchain_address_post(
    configuration: &configuration::Configuration,
    bank_deposit_to_onchain_address_hosted_request: models::BankDepositToOnchainAddressHostedRequest,
    api_signature: Option<&str>,
) -> Result<models::HostedSessionResponse, Error<HostedWorkflowsBankDepositToOnchainAddressPostError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_bank_deposit_to_onchain_address_hosted_request =
        bank_deposit_to_onchain_address_hosted_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/hosted-workflows/bank-deposit-to-onchain-address",
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
    req_builder = req_builder.json(&p_body_bank_deposit_to_onchain_address_hosted_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::HostedSessionResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::HostedSessionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<HostedWorkflowsBankDepositToOnchainAddressPostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint creates a simulated fiat deposit in the sandbox environment for testing purposes.  The solution requires a FiatAmount, `CustomerID` reference, and payment method specifications to trigger configured workflows, rules, and webhook integrations.  Use the endpoint to test automated processes and validate business logic without real money transfers, supporting various fiat currencies and deposit scenarios.  Follow the step-by-step guides:  * [Bank Onramp via USD Virtual Account](../recipes/payin/bank-onramp-us)  * [Bank Onramp via EUR Virtual Account](../recipes/payin/bank-onramp-eu)
pub async fn sandbox_fiat_deposit_simulate_post(
    configuration: &configuration::Configuration,
    fiat_deposit_simulate_request: models::FiatDepositSimulateRequest,
    api_signature: Option<&str>,
) -> Result<models::FiatDepositSimulateResponse, Error<SandboxFiatDepositSimulatePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_fiat_deposit_simulate_request = fiat_deposit_simulate_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!("{}/sandbox/fiat-deposit/simulate", configuration.base_path);
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
    req_builder = req_builder.json(&p_body_fiat_deposit_simulate_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::FiatDepositSimulateResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::FiatDepositSimulateResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SandboxFiatDepositSimulatePostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint establishes an automated workflow that converts incoming cryptocurrency deposits into fiat currency payouts.  The solution requires configuration of source cryptocurrency monitoring, a FormSessionID for payout instructions, recipient payment method details, and conversion parameters, with the deposited amounts traded for the specified fiat currency at current market prices after application of fees.  Use the endpoint to configure SourceAddress specification for deposit tracking, AmountConditions for trigger thresholds, and automatic execution rules with support for multiple cryptocurrencies and global payout channels.  Follow the step-by-step guides:  * [Bank Onramp via USD Virtual Account](../recipes/payin/bank-onramp-us)  * [Bank Onramp via EUR Virtual Account](../recipes/payin/bank-onramp-eu)  **Notes**:   * On-chain operations are irreversible, and market prices may fluctuate significantly, impacting the final payout amount.  * Although this workflow should continue working with the original FormSession, Noah cannot guarantee FormSession longevity. For this reason, generate a new FormSession each time you present the customer with the onchain deposit address.  * If the customer reuses the onchain address after the FormSession becomes invalid, the transfer will fail but funds will not be lost.
pub async fn workflows_bank_deposit_to_onchain_address_post(
    configuration: &configuration::Configuration,
    bank_deposit_to_onchain_address_request: models::BankDepositToOnchainAddressRequest,
    api_signature: Option<&str>,
) -> Result<
    models::BankDepositToOnchainAddressResponse,
    Error<WorkflowsBankDepositToOnchainAddressPostError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_bank_deposit_to_onchain_address_request = bank_deposit_to_onchain_address_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/workflows/bank-deposit-to-onchain-address",
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
    req_builder = req_builder.json(&p_body_bank_deposit_to_onchain_address_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BankDepositToOnchainAddressResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BankDepositToOnchainAddressResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<WorkflowsBankDepositToOnchainAddressPostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
