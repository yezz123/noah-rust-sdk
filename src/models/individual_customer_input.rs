use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualCustomerInput {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "FullName")]
    pub full_name: Box<models::FullName>,
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: String,
    /// Customer's email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "Identities")]
    pub identities: Vec<models::CustomerIdentity>,
    #[serde(rename = "PrimaryResidence")]
    pub primary_residence: Box<models::StreetAddress>,
}

impl IndividualCustomerInput {
    pub fn new(
        r#type: Type,
        full_name: models::FullName,
        date_of_birth: String,
        identities: Vec<models::CustomerIdentity>,
        primary_residence: models::StreetAddress,
    ) -> IndividualCustomerInput {
        IndividualCustomerInput {
            r#type,
            full_name: Box::new(full_name),
            date_of_birth,
            email: None,
            phone_number: None,
            identities,
            primary_residence: Box::new(primary_residence),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Individual")]
    Individual,
}

impl Default for Type {
    fn default() -> Type {
        Self::Individual
    }
}
