use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckoutPayinCryptoPostRequest {
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "CryptoAmount")]
    pub crypto_amount: String,
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    #[serde(rename = "Customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<models::CustomerInput>>,
    #[serde(rename = "LineItems")]
    pub line_items: Vec<models::LineItem>,
    /// A string which must be unique each time a new transaction is created, like a UUID or operation sequence number. Request can be idempotently retried by using the same Nonce.
    #[serde(rename = "Nonce")]
    pub nonce: String,
}

impl CheckoutPayinCryptoPostRequest {
    pub fn new(
        crypto_currency: String,
        crypto_amount: String,
        return_url: String,
        customer_id: String,
        line_items: Vec<models::LineItem>,
        nonce: String,
    ) -> CheckoutPayinCryptoPostRequest {
        CheckoutPayinCryptoPostRequest {
            crypto_currency,
            crypto_amount,
            return_url,
            external_id: None,
            customer_id,
            customer: None,
            line_items,
            nonce,
        }
    }
}
