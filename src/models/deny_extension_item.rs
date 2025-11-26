use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyExtensionItem {
    /// The reason for the access denial. It can be one of the following: * AccountStatus * Compliance  * InsufficientPermissions * VerificationRequired
    #[serde(rename = "Reason")]
    pub reason: String,
    /// A more detailed description of the problem or the fix.
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Principal")]
    pub principal: Box<models::DenyExtensionPrincipal>,
}

impl DenyExtensionItem {
    pub fn new(
        reason: String,
        description: String,
        principal: models::DenyExtensionPrincipal,
    ) -> DenyExtensionItem {
        DenyExtensionItem {
            reason,
            description,
            principal: Box::new(principal),
        }
    }
}
