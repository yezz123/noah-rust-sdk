use crate::models;
use serde::{Deserialize, Serialize};

/// DenyExtensionPrincipal : The principal that was denied access.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenyExtensionPrincipal {
    /// ID of the principal.
    #[serde(rename = "ID")]
    pub id: String,
    /// Type of principal: * User * Customer
    #[serde(rename = "Type")]
    pub r#type: String,
}

impl DenyExtensionPrincipal {
    /// The principal that was denied access.
    pub fn new(id: String, r#type: String) -> DenyExtensionPrincipal {
        DenyExtensionPrincipal { id, r#type }
    }
}
