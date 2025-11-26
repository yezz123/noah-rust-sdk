use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociateInformationInput {
    /// Associate ID.
    #[serde(rename = "ID")]
    pub id: uuid::Uuid,
    /// Relationship types.
    #[serde(rename = "RelationshipTypes")]
    pub relationship_types: Vec<RelationshipTypes>,
    #[serde(rename = "FullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<Box<models::FullName>>,
    #[serde(rename = "DateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "Identities", skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<models::CustomerIdentity>>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "TaxResidenceCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_residence_country: Option<String>,
    /// UBO email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "ResidentialAddress", skip_serializing_if = "Option::is_none")]
    pub residential_address: Option<Box<models::StreetAddress>>,
    #[serde(rename = "UBO", skip_serializing_if = "Option::is_none")]
    pub ubo: Option<Box<models::UboInformationInput>>,
}

impl AssociateInformationInput {
    pub fn new(
        id: uuid::Uuid,
        relationship_types: Vec<RelationshipTypes>,
    ) -> AssociateInformationInput {
        AssociateInformationInput {
            id,
            relationship_types,
            full_name: None,
            date_of_birth: None,
            identities: None,
            tax_residence_country: None,
            email: None,
            phone_number: None,
            residential_address: None,
            ubo: None,
        }
    }
}
/// Relationship types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RelationshipTypes {
    #[serde(rename = "UBO")]
    Ubo,
    #[serde(rename = "Representative")]
    Representative,
}

impl Default for RelationshipTypes {
    fn default() -> RelationshipTypes {
        Self::Ubo
    }
}
