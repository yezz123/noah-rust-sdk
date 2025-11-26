use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UboInformationInput {
    /// Percentage of ownership held by the UBO.
    #[serde(
        rename = "OwnershipPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub ownership_percentage: Option<i32>,
}

impl UboInformationInput {
    pub fn new() -> UboInformationInput {
        UboInformationInput {
            ownership_percentage: None,
        }
    }
}
