use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckoutManageRequest {
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    #[serde(rename = "Customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<models::CustomerInput>>,
}

impl CheckoutManageRequest {
    pub fn new(customer_id: String) -> CheckoutManageRequest {
        CheckoutManageRequest {
            customer_id,
            customer: None,
        }
    }
}
