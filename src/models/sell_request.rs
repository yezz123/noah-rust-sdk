use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SellRequest {
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: String,
    /// Unique identifier for the Form Session. Form Session allows ramping using provided form data.
    #[serde(rename = "FormSessionID")]
    pub form_session_id: uuid::Uuid,
    /// A string which must be unique each time a new transaction is created, like a UUID or operation sequence number. Request can be idempotently retried by using the same Nonce.
    #[serde(rename = "Nonce")]
    pub nonce: String,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl SellRequest {
    pub fn new(
        crypto_currency: String,
        fiat_amount: String,
        crypto_authorized_amount: String,
        form_session_id: uuid::Uuid,
        nonce: String,
    ) -> SellRequest {
        SellRequest {
            crypto_currency,
            fiat_amount,
            crypto_authorized_amount,
            form_session_id,
            nonce,
            external_id: None,
        }
    }
}
