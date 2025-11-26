use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SenderPaymentMethod {
    #[serde(rename = "FullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "Details")]
    pub details: Box<models::PaymentMethodDisplayDetails>,
}

impl SenderPaymentMethod {
    pub fn new(details: models::PaymentMethodDisplayDetails) -> SenderPaymentMethod {
        SenderPaymentMethod {
            full_name: None,
            details: Box::new(details),
        }
    }
}
