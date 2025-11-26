use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomerPrefillScreeningInput {
    /// Does the company integrate an automated or manual screening?
    #[serde(rename = "Method")]
    pub method: Method,
    /// If the method is Automated, please specify the system/software.
    #[serde(rename = "System", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

impl BusinessCustomerPrefillScreeningInput {
    pub fn new(method: Method) -> BusinessCustomerPrefillScreeningInput {
        BusinessCustomerPrefillScreeningInput {
            method,
            system: None,
        }
    }
}
/// Does the company integrate an automated or manual screening?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "Automated")]
    Automated,
    #[serde(rename = "Manual")]
    Manual,
}

impl Default for Method {
    fn default() -> Method {
        Self::Automated
    }
}
