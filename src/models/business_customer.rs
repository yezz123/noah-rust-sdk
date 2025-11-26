use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomer {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    #[serde(rename = "Created")]
    pub created: String,
    /// Name of the business.
    #[serde(rename = "RegisteredName")]
    pub registered_name: String,
    /// Email address of the business.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Registration number of the business.
    #[serde(rename = "RegistrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "RegistrationCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_country: Option<String>,
    #[serde(rename = "RegisteredAddress", skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<Box<models::StreetAddress>>,
    #[serde(rename = "IncorporationDate", skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<String>,
    #[serde(rename = "Verification")]
    pub verification: Box<models::Verification>,
    #[serde(rename = "Verifications", skip_serializing_if = "Option::is_none")]
    pub verifications: Option<Box<models::Verifications>>,
    /// Custom user defined key value pairs used for storing additional information about the customer.
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl BusinessCustomer {
    pub fn new(
        r#type: Type,
        customer_id: String,
        created: String,
        registered_name: String,
        verification: models::Verification,
    ) -> BusinessCustomer {
        BusinessCustomer {
            r#type,
            customer_id,
            created,
            registered_name,
            email: None,
            registration_number: None,
            registration_country: None,
            registered_address: None,
            incorporation_date: None,
            verification: Box::new(verification),
            verifications: None,
            metadata: None,
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
