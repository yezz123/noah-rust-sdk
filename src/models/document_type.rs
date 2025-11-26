use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentType {
    #[serde(rename = "DrivingLicense")]
    DrivingLicense,
    #[serde(rename = "NationalIDCard")]
    NationalIdCard,
    #[serde(rename = "Passport")]
    Passport,
    #[serde(rename = "UtilityBill")]
    UtilityBill,
    #[serde(rename = "ResidencePermit")]
    ResidencePermit,
    #[serde(rename = "ForeignerID")]
    ForeignerId,
    #[serde(rename = "MemorandumAndArticles")]
    MemorandumAndArticles,
    #[serde(rename = "ShareholdersExtract")]
    ShareholdersExtract,
    #[serde(rename = "RegistryExtract")]
    RegistryExtract,
    #[serde(rename = "DirectorsExtract")]
    DirectorsExtract,
    #[serde(rename = "OngoingMonitoringProcedure")]
    OngoingMonitoringProcedure,
    #[serde(rename = "PEPSanctionsAdverseMediaProcedure")]
    PepSanctionsAdverseMediaProcedure,
    #[serde(rename = "OnboardingKYCAndCIPProcedure")]
    OnboardingKycAndCipProcedure,
    #[serde(rename = "CDDEDDProcedure")]
    CddeddProcedure,
    #[serde(rename = "SARSTRReportingProcedure")]
    SarstrReportingProcedure,
    #[serde(rename = "AntiBriberyCorruptionProcedure")]
    AntiBriberyCorruptionProcedure,
    #[serde(rename = "AMLTrainingScheduleAndRecords")]
    AmlTrainingScheduleAndRecords,
    #[serde(rename = "CorporateShareholderExtract")]
    CorporateShareholderExtract,
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DrivingLicense => write!(f, "DrivingLicense"),
            Self::NationalIdCard => write!(f, "NationalIDCard"),
            Self::Passport => write!(f, "Passport"),
            Self::UtilityBill => write!(f, "UtilityBill"),
            Self::ResidencePermit => write!(f, "ResidencePermit"),
            Self::ForeignerId => write!(f, "ForeignerID"),
            Self::MemorandumAndArticles => write!(f, "MemorandumAndArticles"),
            Self::ShareholdersExtract => write!(f, "ShareholdersExtract"),
            Self::RegistryExtract => write!(f, "RegistryExtract"),
            Self::DirectorsExtract => write!(f, "DirectorsExtract"),
            Self::OngoingMonitoringProcedure => write!(f, "OngoingMonitoringProcedure"),
            Self::PepSanctionsAdverseMediaProcedure => {
                write!(f, "PEPSanctionsAdverseMediaProcedure")
            }
            Self::OnboardingKycAndCipProcedure => write!(f, "OnboardingKYCAndCIPProcedure"),
            Self::CddeddProcedure => write!(f, "CDDEDDProcedure"),
            Self::SarstrReportingProcedure => write!(f, "SARSTRReportingProcedure"),
            Self::AntiBriberyCorruptionProcedure => write!(f, "AntiBriberyCorruptionProcedure"),
            Self::AmlTrainingScheduleAndRecords => write!(f, "AMLTrainingScheduleAndRecords"),
            Self::CorporateShareholderExtract => write!(f, "CorporateShareholderExtract"),
        }
    }
}

impl Default for DocumentType {
    fn default() -> DocumentType {
        Self::DrivingLicense
    }
}
