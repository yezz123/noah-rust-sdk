use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckoutSession {
    /// Unique ID of session.
    #[serde(rename = "CheckoutSessionID")]
    pub checkout_session_id: String,
    /// Categorizes one or more `PaymentMethodType`s into broader groups. Useful for listing channels, displaying payment methods:  * Bank  * Card  * Identifier
    #[serde(
        rename = "PaymentMethodCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_method_category: Option<String>,
    /// A scalar representing a financial asset code. Suitable for use in cases where exactly one of the currency codes must be provided.
    #[serde(rename = "SourceCurrency")]
    pub source_currency: String,
    /// A scalar representing a financial asset code. Suitable for use in cases where exactly one of the currency codes must be provided.
    #[serde(rename = "DestinationCurrency")]
    pub destination_currency: String,
    #[serde(rename = "SourceAmount", skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<String>,
    #[serde(rename = "DestinationAmount", skip_serializing_if = "Option::is_none")]
    pub destination_amount: Option<String>,
    #[serde(rename = "AuthorizedAmount", skip_serializing_if = "Option::is_none")]
    pub authorized_amount: Option<String>,
    /// Status of CheckoutSession: * Pending * Failed * Settled  Note: other statuses could be added in the future
    #[serde(rename = "Status")]
    pub status: String,
    /// Unique identifier in user's system.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    #[serde(rename = "LineItems")]
    pub line_items: Vec<models::LineItem>,
    /// Type of checkout session: * PayinCrypto * PayinFiat * PayoutFiat
    #[serde(rename = "Type")]
    pub r#type: String,
    /// Time the checkout expires (example: 2020-01-01T00:00:00Z)
    #[serde(rename = "Expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Time the checkout was created (example: 2020-01-01T00:00:00Z)
    #[serde(rename = "Created")]
    pub created: String,
}

impl CheckoutSession {
    pub fn new(
        checkout_session_id: String,
        source_currency: String,
        destination_currency: String,
        status: String,
        customer_id: String,
        return_url: String,
        line_items: Vec<models::LineItem>,
        r#type: String,
        created: String,
    ) -> CheckoutSession {
        CheckoutSession {
            checkout_session_id,
            payment_method_category: None,
            source_currency,
            destination_currency,
            source_amount: None,
            destination_amount: None,
            authorized_amount: None,
            status,
            external_id: None,
            customer_id,
            return_url,
            line_items,
            r#type,
            expiry: None,
            created,
        }
    }
}
