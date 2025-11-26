use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SellActionInput {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Cryptocurrency code or output reference.
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "CryptoAuthorizedAmount")]
    pub crypto_authorized_amount: Box<models::StepDecimal>,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: Box<models::StepDecimal>,
    /// Unique identifier for the Form Session. Form Session allows ramping using provided form data.
    #[serde(rename = "FormSessionID")]
    pub form_session_id: uuid::Uuid,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl SellActionInput {
    pub fn new(
        r#type: Type,
        crypto_currency: String,
        crypto_authorized_amount: models::StepDecimal,
        fiat_amount: models::StepDecimal,
        form_session_id: uuid::Uuid,
    ) -> SellActionInput {
        SellActionInput {
            r#type,
            crypto_currency,
            crypto_authorized_amount: Box::new(crypto_authorized_amount),
            fiat_amount: Box::new(fiat_amount),
            form_session_id,
            external_id: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SellActionInput")]
    SellActionInput,
}

impl Default for Type {
    fn default() -> Type {
        Self::SellActionInput
    }
}
