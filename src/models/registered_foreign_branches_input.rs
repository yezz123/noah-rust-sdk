use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredForeignBranchesInput {
    /// Name of the registered foreign branch.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "RegistrationCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_country: Option<String>,
    /// Registration number of the registered foreign branch.
    #[serde(rename = "RegistrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
}

impl RegisteredForeignBranchesInput {
    pub fn new() -> RegisteredForeignBranchesInput {
        RegisteredForeignBranchesInput {
            name: None,
            registration_country: None,
            registration_number: None,
        }
    }
}
