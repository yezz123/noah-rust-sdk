use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckoutSessionResponse {
    /// URL to redirect the customer to the checkout page.
    #[serde(rename = "HostedURL")]
    pub hosted_url: String,
    #[serde(rename = "CheckoutSession")]
    pub checkout_session: Box<models::CheckoutSession>,
}

impl CheckoutSessionResponse {
    pub fn new(
        hosted_url: String,
        checkout_session: models::CheckoutSession,
    ) -> CheckoutSessionResponse {
        CheckoutSessionResponse {
            hosted_url,
            checkout_session: Box::new(checkout_session),
        }
    }
}
