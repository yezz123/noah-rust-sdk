use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "ID")]
    pub id: String,
    /// Categorizes one or more `PaymentMethodType`s into broader groups. Useful for listing channels, displaying payment methods:  * Bank  * Card  * Identifier
    #[serde(rename = "PaymentMethodCategory")]
    pub payment_method_category: String,
    /// Specific type of payment method:  * BankSepa  * BankLocal  * BankFedwire  * TokenizedCard  * IdentifierPix
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: String,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Calculated", skip_serializing_if = "Option::is_none")]
    pub calculated: Option<Box<models::ChannelCalculated>>,
    #[serde(rename = "Limits")]
    pub limits: Box<models::ChannelLimits>,
    #[serde(rename = "Rate")]
    pub rate: String,
    /// Recent payment methods for the channel. Only returned if `CustomerID` was provided in the query.
    #[serde(rename = "PaymentMethods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<models::PaymentMethodDisplay>>,
    /// The time it takes to process a transaction through the channel
    #[serde(rename = "ProcessingSeconds")]
    pub processing_seconds: i32,
    #[serde(rename = "ProcessingTier", skip_serializing_if = "Option::is_none")]
    pub processing_tier: Option<models::ProcessingTier>,
    #[serde(rename = "FormSchema", skip_serializing_if = "Option::is_none")]
    pub form_schema: Option<Box<models::FormSchema>>,
}

impl Channel {
    pub fn new(
        id: String,
        payment_method_category: String,
        payment_method_type: String,
        fiat_currency: String,
        country: String,
        limits: models::ChannelLimits,
        rate: String,
        processing_seconds: i32,
    ) -> Channel {
        Channel {
            id,
            payment_method_category,
            payment_method_type,
            fiat_currency,
            country,
            calculated: None,
            limits: Box::new(limits),
            rate,
            payment_methods: None,
            processing_seconds,
            processing_tier: None,
            form_schema: None,
        }
    }
}
