use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnchainDepositToPaymentMethodRequest {
    #[serde(rename = "Trigger")]
    pub trigger: Box<models::SingleOnchainDepositSourceTriggerInput>,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,
    /// Unique identifier for the Form Session. Form Session allows ramping using provided form data.
    #[serde(rename = "FormSessionID")]
    pub form_session_id: uuid::Uuid,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl OnchainDepositToPaymentMethodRequest {
    pub fn new(
        trigger: models::SingleOnchainDepositSourceTriggerInput,
        customer_id: String,
        crypto_currency: String,
        fiat_amount: String,
        form_session_id: uuid::Uuid,
    ) -> OnchainDepositToPaymentMethodRequest {
        OnchainDepositToPaymentMethodRequest {
            trigger: Box::new(trigger),
            customer_id,
            crypto_currency,
            fiat_amount,
            form_session_id,
            external_id: None,
        }
    }
}
