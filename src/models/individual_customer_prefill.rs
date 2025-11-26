use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualCustomerPrefill {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "FullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<Box<models::FullName>>,
    #[serde(rename = "DateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "Identities", skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<models::CustomerIdentity>>,
    #[serde(rename = "PrimaryResidence", skip_serializing_if = "Option::is_none")]
    pub primary_residence: Option<Box<models::StreetAddress>>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "Citizenship", skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "TaxResidenceCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_residence_country: Option<String>,
    /// Customer's email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// What's your main source of income?
    #[serde(rename = "SourceOfIncome", skip_serializing_if = "Option::is_none")]
    pub source_of_income: Option<SourceOfIncome>,
    /// What's your employment status?
    #[serde(rename = "EmploymentStatus", skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<EmploymentStatus>,
    /// What industry do you work in?
    #[serde(rename = "WorkIndustry", skip_serializing_if = "Option::is_none")]
    pub work_industry: Option<WorkIndustry>,
    #[serde(rename = "FinancialsUsd", skip_serializing_if = "Option::is_none")]
    pub financials_usd: Option<Box<models::IndividualFinancialsInput>>,
    /// What's your expected frequency of transactions?
    #[serde(
        rename = "TransactionFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_frequency: Option<TransactionFrequency>,
}

impl IndividualCustomerPrefill {
    pub fn new(r#type: Type) -> IndividualCustomerPrefill {
        IndividualCustomerPrefill {
            r#type,
            full_name: None,
            date_of_birth: None,
            identities: None,
            primary_residence: None,
            citizenship: None,
            tax_residence_country: None,
            email: None,
            phone_number: None,
            source_of_income: None,
            employment_status: None,
            work_industry: None,
            financials_usd: None,
            transaction_frequency: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "IndividualCustomerPrefill")]
    IndividualCustomerPrefill,
}

impl Default for Type {
    fn default() -> Type {
        Self::IndividualCustomerPrefill
    }
}
/// What's your main source of income?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceOfIncome {
    #[serde(rename = "Salary")]
    Salary,
    #[serde(rename = "Pension")]
    Pension,
    #[serde(rename = "Investment")]
    Investment,
    #[serde(rename = "Property")]
    Property,
    #[serde(rename = "FriendsAndFamily")]
    FriendsAndFamily,
    #[serde(rename = "Benefits")]
    Benefits,
}

impl Default for SourceOfIncome {
    fn default() -> SourceOfIncome {
        Self::Salary
    }
}
/// What's your employment status?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmploymentStatus {
    #[serde(rename = "Employed")]
    Employed,
    #[serde(rename = "Unemployed")]
    Unemployed,
    #[serde(rename = "Retired")]
    Retired,
    #[serde(rename = "Student")]
    Student,
    #[serde(rename = "SelfEmployed")]
    SelfEmployed,
}

impl Default for EmploymentStatus {
    fn default() -> EmploymentStatus {
        Self::Employed
    }
}
/// What industry do you work in?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkIndustry {
    #[serde(rename = "BankingAndFinancialServices")]
    BankingAndFinancialServices,
    #[serde(rename = "InvestmentAndSecurities")]
    InvestmentAndSecurities,
    #[serde(rename = "Insurance")]
    Insurance,
    #[serde(rename = "RealEstate")]
    RealEstate,
    #[serde(rename = "LegalServices")]
    LegalServices,
    #[serde(rename = "AccountingAndAuditing")]
    AccountingAndAuditing,
    #[serde(rename = "GamingAndGambling")]
    GamingAndGambling,
    #[serde(rename = "MiningAndEnergy")]
    MiningAndEnergy,
    #[serde(rename = "RetailAndECommerce")]
    RetailAndECommerce,
    #[serde(rename = "HealthcareAndPharmaceuticals")]
    HealthcareAndPharmaceuticals,
    #[serde(rename = "GovernmentAndPublicSector")]
    GovernmentAndPublicSector,
    #[serde(rename = "NonProfitAndCharity")]
    NonProfitAndCharity,
    #[serde(rename = "TechnologyAndSoftwareDevelopment")]
    TechnologyAndSoftwareDevelopment,
    #[serde(rename = "TransportationAndLogistics")]
    TransportationAndLogistics,
    #[serde(rename = "HospitalityAndTourism")]
    HospitalityAndTourism,
}

impl Default for WorkIndustry {
    fn default() -> WorkIndustry {
        Self::BankingAndFinancialServices
    }
}
/// What's your expected frequency of transactions?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionFrequency {
    #[serde(rename = "OncePerYear")]
    OncePerYear,
    #[serde(rename = "OnceEveryFewMonths")]
    OnceEveryFewMonths,
    #[serde(rename = "AFewTimesPerMonth")]
    AFewTimesPerMonth,
    #[serde(rename = "OncePerWeek")]
    OncePerWeek,
    #[serde(rename = "MoreThanOncePerWeek")]
    MoreThanOncePerWeek,
}

impl Default for TransactionFrequency {
    fn default() -> TransactionFrequency {
        Self::OncePerYear
    }
}
