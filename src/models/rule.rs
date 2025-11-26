use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "Trigger")]
    pub trigger: Box<models::RuleTrigger>,
    #[serde(rename = "ID")]
    pub id: uuid::Uuid,
}

impl Rule {
    pub fn new(trigger: models::RuleTrigger, id: uuid::Uuid) -> Rule {
        Rule {
            trigger: Box::new(trigger),
            id,
        }
    }
}
