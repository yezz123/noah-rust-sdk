use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    /// Type of the event being sent:  * Transaction  * FiatDeposit  * CheckoutSession
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// Version of the event
    #[serde(rename = "EventVersion")]
    pub event_version: i64,
    #[serde(rename = "Occurred")]
    pub occurred: String,
    #[serde(rename = "Data")]
    pub data: Box<models::WebhookData>,
    /// User ID of the event.
    #[serde(rename = "UserID")]
    pub user_id: String,
}

impl Webhook {
    pub fn new(
        event_type: String,
        event_version: i64,
        occurred: String,
        data: models::WebhookData,
        user_id: String,
    ) -> Webhook {
        Webhook {
            event_type,
            event_version,
            occurred,
            data: Box::new(data),
            user_id,
        }
    }
}
