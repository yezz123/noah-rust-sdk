use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "ID")]
    pub id: String,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "Country")]
    pub country: String,
    /// Categorizes one or more `PaymentMethodType`s into broader groups. Useful for listing channels, displaying payment methods:  * Bank  * Card  * Identifier
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: String,
    #[serde(rename = "DisplayDetails")]
    pub display_details: Box<models::PaymentMethodDisplayDetails>,
}

impl PaymentMethod {
    pub fn new(
        id: String,
        country: String,
        payment_method_category: String,
        display_details: models::PaymentMethodDisplayDetails,
    ) -> PaymentMethod {
        PaymentMethod {
            id,
            customer_id: None,
            country,
            payment_method_category,
            display_details: Box::new(display_details),
        }
    }
}
