use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankDepositToOnchainAddressResponse {
    /// Existing payment method id to be used.
    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: String,
    /// Specific type of payment method:  * BankSepa  * BankLocal  * BankFedwire  * TokenizedCard  * IdentifierPix
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: String,
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "AccountHolderName", skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "BankName", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "BankAddress", skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<Box<models::StreetAddress>>,
}

impl BankDepositToOnchainAddressResponse {
    pub fn new(
        payment_method_id: String,
        payment_method_type: String,
        account_number: String,
    ) -> BankDepositToOnchainAddressResponse {
        BankDepositToOnchainAddressResponse {
            payment_method_id,
            payment_method_type,
            reference: None,
            account_number,
            account_holder_name: None,
            bank_code: None,
            bank_name: None,
            bank_address: None,
        }
    }
}
