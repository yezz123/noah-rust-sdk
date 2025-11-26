use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomerInput {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Name of the business.
    #[serde(rename = "RegisteredName")]
    pub registered_name: String,
    /// Email address of the business.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Registration number of the business.
    #[serde(rename = "RegistrationNumber")]
    pub registration_number: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "RegistrationCountry")]
    pub registration_country: String,
    #[serde(rename = "RegisteredAddress")]
    pub registered_address: Box<models::StreetAddress>,
    #[serde(rename = "IncorporationDate")]
    pub incorporation_date: String,
}

impl BusinessCustomerInput {
    pub fn new(
        r#type: Type,
        registered_name: String,
        registration_number: String,
        registration_country: String,
        registered_address: models::StreetAddress,
        incorporation_date: String,
    ) -> BusinessCustomerInput {
        BusinessCustomerInput {
            r#type,
            registered_name,
            email: None,
            registration_number,
            registration_country,
            registered_address: Box::new(registered_address),
            incorporation_date,
        }
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
