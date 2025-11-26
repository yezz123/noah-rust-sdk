use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodDisplay {
    #[serde(rename = "ID")]
    pub id: String,
    /// Specific type of payment method:  * BankSepa  * BankLocal  * BankFedwire  * TokenizedCard  * IdentifierPix
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: String,
    #[serde(rename = "Details")]
    pub details: Box<models::PaymentMethodDisplayDetails>,
}

impl PaymentMethodDisplay {
    pub fn new(
        id: String,
        payment_method_type: String,
        details: models::PaymentMethodDisplayDetails,
    ) -> PaymentMethodDisplay {
        PaymentMethodDisplay {
            id,
            payment_method_type,
            details: Box::new(details),
        }
    }
}
