use crate::models;
use serde::{Deserialize, Serialize};

/// Verifications : Verification data for the customer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verifications {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "EntityVerifications")]
    pub entity_verifications: Vec<models::EntityVerification>,
}

impl Verifications {
    /// Verification data for the customer
    pub fn new(
        r#type: Type,
        status: String,
        entity_verifications: Vec<models::EntityVerification>,
    ) -> Verifications {
        Verifications {
            r#type,
            status,
            entity_verifications,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Individual")]
    Individual,
    #[serde(rename = "Business")]
    Business,
}

impl Default for Type {
    fn default() -> Type {
        Self::Individual
    }
}
