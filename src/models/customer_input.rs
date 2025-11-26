use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum CustomerInput {
    #[serde(rename = "Individual")]
    Individual(Box<models::IndividualCustomerInput>),
    #[serde(rename = "Business")]
    Business(Box<models::BusinessCustomerInput>),
}

impl Default for CustomerInput {
    fn default() -> Self {
        Self::Individual(Default::default())
    }
}
