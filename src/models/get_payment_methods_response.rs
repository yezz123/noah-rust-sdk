use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPaymentMethodsResponse {
    #[serde(rename = "Items")]
    pub items: Vec<models::PaymentMethod>,
    #[serde(rename = "PageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetPaymentMethodsResponse {
    pub fn new(items: Vec<models::PaymentMethod>) -> GetPaymentMethodsResponse {
        GetPaymentMethodsResponse {
            items,
            page_token: None,
        }
    }
}
