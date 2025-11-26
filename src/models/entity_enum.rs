use crate::models;
use serde::{Deserialize, Serialize};

/// EntityEnum : Entity type for business entities
/// Entity type for business entities
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityEnum {
    #[serde(rename = "LimitedLiabilityCompany")]
    LimitedLiabilityCompany,
    #[serde(rename = "PublicCompany")]
    PublicCompany,
    #[serde(rename = "SoleProprietorship")]
    SoleProprietorship,
    #[serde(rename = "Partnership")]
    Partnership,
    #[serde(rename = "Corporation")]
    Corporation,
    #[serde(rename = "Trust")]
    Trust,
    #[serde(rename = "PrivateFoundation")]
    PrivateFoundation,
    #[serde(rename = "Charity")]
    Charity,
    #[serde(rename = "NonProfitOrganization")]
    NonProfitOrganization,
    #[serde(rename = "PublicAgency")]
    PublicAgency,
}

impl std::fmt::Display for EntityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LimitedLiabilityCompany => write!(f, "LimitedLiabilityCompany"),
            Self::PublicCompany => write!(f, "PublicCompany"),
            Self::SoleProprietorship => write!(f, "SoleProprietorship"),
            Self::Partnership => write!(f, "Partnership"),
            Self::Corporation => write!(f, "Corporation"),
            Self::Trust => write!(f, "Trust"),
            Self::PrivateFoundation => write!(f, "PrivateFoundation"),
            Self::Charity => write!(f, "Charity"),
            Self::NonProfitOrganization => write!(f, "NonProfitOrganization"),
            Self::PublicAgency => write!(f, "PublicAgency"),
        }
    }
}

impl Default for EntityEnum {
    fn default() -> EntityEnum {
        Self::LimitedLiabilityCompany
    }
}
