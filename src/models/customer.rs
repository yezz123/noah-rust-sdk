use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Customer {
    #[serde(rename = "Individual")]
    Individual(Box<models::IndividualCustomer>),
    #[serde(rename = "Business")]
    Business(Box<models::BusinessCustomer>),
}

impl Default for Customer {
    fn default() -> Self {
        Self::Individual(Default::default())
    }
}
