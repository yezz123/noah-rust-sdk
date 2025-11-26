use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualCustomer {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: String,
    #[serde(rename = "FullName")]
    pub full_name: Box<models::FullName>,
    #[serde(rename = "Identities")]
    pub identities: Vec<models::CustomerIdentity>,
    #[serde(rename = "PrimaryResidence")]
    pub primary_residence: Box<models::StreetAddress>,
    #[serde(rename = "Verification")]
    pub verification: Box<models::Verification>,
    #[serde(rename = "Verifications", skip_serializing_if = "Option::is_none")]
    pub verifications: Option<Box<models::Verifications>>,
    /// Custom user defined key value pairs used for storing additional information about the customer.
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl IndividualCustomer {
    pub fn new(
        r#type: Type,
        customer_id: String,
        created: String,
        date_of_birth: String,
        full_name: models::FullName,
        identities: Vec<models::CustomerIdentity>,
        primary_residence: models::StreetAddress,
        verification: models::Verification,
    ) -> IndividualCustomer {
        IndividualCustomer {
            r#type,
            customer_id,
            created,
            date_of_birth,
            full_name: Box::new(full_name),
            identities,
            primary_residence: Box::new(primary_residence),
            verification: Box::new(verification),
            verifications: None,
            metadata: None,
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
