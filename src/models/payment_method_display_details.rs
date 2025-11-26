use crate::models;
use serde::{Deserialize, Serialize};

/// PaymentMethodDisplayDetails : Contains information to display each payment method, with fields tailored to the specific type (e.g., last four digits or account number) to help identify the payment method.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum PaymentMethodDisplayDetails {
    #[serde(rename = "FiatPaymentMethodBankDisplay")]
    FiatPaymentMethodBankDisplay(Box<models::FiatPaymentMethodBankDisplay>),
    #[serde(rename = "FiatPaymentMethodCardDisplay")]
    FiatPaymentMethodCardDisplay(Box<models::FiatPaymentMethodCardDisplay>),
    #[serde(rename = "FiatPaymentMethodIdentifierDisplay")]
    FiatPaymentMethodIdentifierDisplay(Box<models::FiatPaymentMethodIdentifierDisplay>),
}

impl Default for PaymentMethodDisplayDetails {
    fn default() -> Self {
        Self::FiatPaymentMethodBankDisplay(Default::default())
    }
}
