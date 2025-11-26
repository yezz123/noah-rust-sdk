use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentSide {
    #[serde(rename = "Front")]
    Front,
    #[serde(rename = "Back")]
    Back,
}

impl std::fmt::Display for DocumentSide {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Front => write!(f, "Front"),
            Self::Back => write!(f, "Back"),
        }
    }
}

impl Default for DocumentSide {
    fn default() -> DocumentSide {
        Self::Front
    }
}
