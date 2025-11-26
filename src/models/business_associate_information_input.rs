use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessAssociateInformationInput {
    /// Shareholder ID.
    #[serde(rename = "ID")]
    pub id: uuid::Uuid,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "RegistrationCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_country: Option<String>,
    /// Shareholders company name.
    #[serde(rename = "CompanyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Shareholders registration number.
    #[serde(rename = "RegistrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename = "EntityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<models::EntityEnum>,
    #[serde(rename = "IncorporationDate", skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<String>,
    /// Shareholders ownership percentage.
    #[serde(
        rename = "OwnershipPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub ownership_percentage: Option<i32>,
}

impl BusinessAssociateInformationInput {
    pub fn new(id: uuid::Uuid) -> BusinessAssociateInformationInput {
        BusinessAssociateInformationInput {
            id,
            registration_country: None,
            company_name: None,
            registration_number: None,
            entity_type: None,
            incorporation_date: None,
            ownership_percentage: None,
        }
    }
}
