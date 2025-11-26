use crate::models;
use serde::{Deserialize, Serialize};

/// WebhookData : Data specific to the event.
/// Data specific to the event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookData {
    Transaction(Box<models::Transaction>),
    FiatDeposit(Box<models::FiatDeposit>),
    CheckoutSession(Box<models::CheckoutSession>),
    Customer(Box<models::Customer>),
}

impl Default for WebhookData {
    fn default() -> Self {
        Self::Transaction(Default::default())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Business")]
    Business,
}

impl Default for Type {
    fn default() -> Type {
        Self::Business
    }
}
