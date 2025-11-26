use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullName {
    /// user's first name
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// user's last name (family name)
    #[serde(rename = "LastName")]
    pub last_name: String,
    /// user's middle name
    #[serde(rename = "MiddleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
}

impl FullName {
    pub fn new(first_name: String, last_name: String) -> FullName {
        FullName {
            first_name,
            last_name,
            middle_name: None,
        }
    }
}
