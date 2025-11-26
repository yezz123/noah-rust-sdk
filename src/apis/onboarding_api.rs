use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`customers_customer_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomersCustomerIdPutError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`onboarding_customer_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnboardingCustomerIdPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`onboarding_customer_id_prefill_documents_upload_url_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnboardingCustomerIdPrefillDocumentsUploadUrlGetError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`onboarding_customer_id_prefill_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnboardingCustomerIdPrefillPostError {
    Status400(models::Error),
    Status401(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// This endpoint initiates [Noah's Reliance Model onboarding flow](../getting-started/kyc#reliance-model). Customers created from this endpoint must have a valid KYC status at the point of passing this data to Noah, and when executing a transaction. A unique `CustomerID` must be provided to enable unique identification in Noah. Customers are approved synchronously from the API call, and are immediately available to transact.  Before you get started with this endpoint, Noah must have authorized your usage of the Reliance Model. For more details on this process, see the [Compliance Overview](../getting-started/kyc).  Follow the step-by-step guide: [Reliance Onboarding Recipe](../recipes/onboarding/reliance-onboarding)  Note: Customers created from this endpoint are not able to process USD payments. In this case, follow the step-by-step guide for onboarding for USD payments: [Hosted Onboarding Recipe](../recipes/onboarding/hosted-onboarding)
pub async fn customers_customer_id_put(
    configuration: &configuration::Configuration,
    customer_id: &str,
    customer_input: models::CustomerInput,
    api_signature: Option<&str>,
) -> Result<(), Error<CustomersCustomerIdPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_customer_id = customer_id;
    let p_body_customer_input = customer_input;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/customers/{CustomerID}",
        configuration.base_path,
        CustomerID = crate::apis::urlencode(p_path_customer_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_body_customer_input);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CustomersCustomerIdPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint initiates [Noah's Standard Model onboarding flow](../getting-started/kyc#standard-model). Through this solution, Noah establishes direct contractual relationships with your end customers, handling all KYB/KYC verification and Tems and Conditions acceptance on your behalf.  Use the endpoint to retrieve a response consisting of a URL, which you pass to your customer so that they can enter their details in a Hosted Onboarding session.  Follow the step-by-step guide: [Hosted Onboarding Recipe](../recipes/onboarding/hosted-onboarding)  Note: Customers needing to process USD payments must make use of Hosted Onboarding, rather than using [Reliance Onboarding](../api-reference/create-update-customer).
pub async fn onboarding_customer_id_post(
    configuration: &configuration::Configuration,
    customer_id: &str,
    hosted_onboarding_request: models::HostedOnboardingRequest,
    api_signature: Option<&str>,
) -> Result<models::HostedSessionResponse, Error<OnboardingCustomerIdPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_customer_id = customer_id;
    let p_body_hosted_onboarding_request = hosted_onboarding_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/onboarding/{CustomerID}",
        configuration.base_path,
        CustomerID = crate::apis::urlencode(p_path_customer_id)
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
    req_builder = req_builder.json(&p_body_hosted_onboarding_request);

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
        let entity: Option<OnboardingCustomerIdPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a URL to upload a document programmatically to add documents to KYC onboarding flows.  Multiple types of documents are supported, using the `Type` parameter described below, both for individual users and company documents for business users.  The response consists of a URL, together with the date and time at which it will expire, which should be used like this, for example:  `curl -X PUT -H 'Content-Type: image/png' --upload-file image.png '{{URL}}'`  **Note**: If a document is double-sided, submit two images and set up the optional `Side` property (`Front` and `Back`).
pub async fn onboarding_customer_id_prefill_documents_upload_url_get(
    configuration: &configuration::Configuration,
    customer_id: &str,
    r#type: models::DocumentType,
    country_code: &str,
    side: Option<models::DocumentSide>,
    associate_id: Option<&str>,
    api_signature: Option<&str>,
) -> Result<
    models::PrefillDocumentUploadUrlResponse,
    Error<OnboardingCustomerIdPrefillDocumentsUploadUrlGetError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_customer_id = customer_id;
    let p_query_type = r#type;
    let p_query_country_code = country_code;
    let p_query_side = side;
    let p_query_associate_id = associate_id;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/onboarding/{CustomerID}/prefill/documents/upload-url",
        configuration.base_path,
        CustomerID = crate::apis::urlencode(p_path_customer_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("Type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_side {
        req_builder = req_builder.query(&[("Side", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("CountryCode", &p_query_country_code.to_string())]);
    if let Some(ref param_value) = p_query_associate_id {
        req_builder = req_builder.query(&[("AssociateID", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PrefillDocumentUploadUrlResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PrefillDocumentUploadUrlResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<OnboardingCustomerIdPrefillDocumentsUploadUrlGetError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Use the Prefill Customer Details endpoint to create a Customer and submit any KYC/KYB information you have already collected.  Prefill allows you to pre-populate known data before initiating the [Hosted Onboarding](../recipes/onboarding/hosted-onboarding) flow, which will gather any remaining required information to collect missing compliance data through dynamic forms or hosted sessions, Terms and Conditions acceptance for regulatory compliance, and Fiat currency selection.  The `Type` parameter defines the known data for prefilling a customer's Hosted Onboarding session, which can be one of the following.  * **`SumSubToken`.** Follow the step-by-step guide: [Token Share Onboarding Recipe](../recipes/onboarding/token-share-onboarding).  * **`BusinessCustomerPrefill`.** Follow the step-by-step guide: [Business Customer Prefill Recipe](../recipes/onboarding/business-customer-prefill).  * **`IndividualCustomerPrefill`.** Follow the step-by-step guide: [Individual Customer Prefill Recipe](../recipes/onboarding/individual-customer-prefill)
pub async fn onboarding_customer_id_prefill_post(
    configuration: &configuration::Configuration,
    customer_id: &str,
    prefill_onboarding_request: models::PrefillOnboardingRequest,
    api_signature: Option<&str>,
) -> Result<(), Error<OnboardingCustomerIdPrefillPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_customer_id = customer_id;
    let p_body_prefill_onboarding_request = prefill_onboarding_request;
    let p_header_api_signature = api_signature;

    let uri_str = format!(
        "{}/onboarding/{CustomerID}/prefill",
        configuration.base_path,
        CustomerID = crate::apis::urlencode(p_path_customer_id)
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
    req_builder = req_builder.json(&p_body_prefill_onboarding_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<OnboardingCustomerIdPrefillPostError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
