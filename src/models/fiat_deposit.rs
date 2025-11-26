use crate::models;
use serde::{Deserialize, Serialize};

/// FiatDeposit : Details of a fiat deposit received by NOAH.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatDeposit {
    /// Unique identifier of the fiat deposit.
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "FiatAmount")]
    pub fiat_amount: String,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
    /// Reference of the deposit.
    #[serde(rename = "Reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "Status")]
    pub status: models::FiatDepositStatus,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Existing payment method id to be used.
    #[serde(rename = "PaymentMethodID")]
    pub payment_method_id: String,
    /// Specific type of payment method:  * BankSepa  * BankLocal  * BankFedwire  * TokenizedCard  * IdentifierPix
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: String,
    /// The transaction ID in the payment system (e.g., IMAD for Wire, UETR for Swift, Trace Number for ACH).
    #[serde(rename = "PaymentSystemID", skip_serializing_if = "Option::is_none")]
    pub payment_system_id: Option<String>,
    #[serde(rename = "Sender")]
    pub sender: Box<models::SenderPaymentMethod>,
    /// Fiat deposit refunds if any exists
    #[serde(rename = "Refunds")]
    pub refunds: Vec<models::FiatPaymentRefund>,
}

impl FiatDeposit {
    /// Details of a fiat deposit received by NOAH.
    pub fn new(
        id: String,
        created: String,
        fiat_amount: String,
        fiat_currency: String,
        status: models::FiatDepositStatus,
        payment_method_id: String,
        payment_method_type: String,
        sender: models::SenderPaymentMethod,
        refunds: Vec<models::FiatPaymentRefund>,
    ) -> FiatDeposit {
        FiatDeposit {
            id,
            created,
            fiat_amount,
            fiat_currency,
            reference: None,
            status,
            customer_id: None,
            payment_method_id,
            payment_method_type,
            payment_system_id: None,
            sender: Box::new(sender),
            refunds,
        }
    }
}
