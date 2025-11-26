use crate::models;
use serde::{Deserialize, Serialize};

/// EntityVerification : Entity-specific verification data for the customer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityVerification {
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Modified")]
    pub modified: String,
    #[serde(rename = "Entity")]
    pub entity: String,
    #[serde(rename = "RejectionData", skip_serializing_if = "Option::is_none")]
    pub rejection_data: Option<Box<models::EntityRejectionData>>,
}

impl EntityVerification {
    /// Entity-specific verification data for the customer
    pub fn new(
        model: String,
        status: String,
        created: String,
        modified: String,
        entity: String,
    ) -> EntityVerification {
        EntityVerification {
            model,
            status,
            created,
            modified,
            entity,
            rejection_data: None,
        }
    }
}
