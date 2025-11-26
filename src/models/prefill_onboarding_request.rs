use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum PrefillOnboardingRequest {
    #[serde(rename = "SumSubToken")]
    SumSubToken(Box<models::SumSubToken>),
    #[serde(rename = "BusinessCustomerPrefill")]
    BusinessCustomerPrefill(Box<models::BusinessCustomerPrefill>),
    #[serde(rename = "IndividualCustomerPrefill")]
    IndividualCustomerPrefill(Box<models::IndividualCustomerPrefill>),
}

impl Default for PrefillOnboardingRequest {
    fn default() -> Self {
        Self::SumSubToken(Default::default())
    }
}

/// Ownership type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OwnershipType {
    #[serde(rename = "Private")]
    Private,
    #[serde(rename = "PublicListed")]
    PublicListed,
    #[serde(rename = "GovernmentOwned")]
    GovernmentOwned,
    #[serde(rename = "Partnership")]
    Partnership,
    #[serde(rename = "SoleProprietorship")]
    SoleProprietorship,
    #[serde(rename = "NonProfit")]
    NonProfit,
    #[serde(rename = "Cooperative")]
    Cooperative,
    #[serde(rename = "JointVenture")]
    JointVenture,
    #[serde(rename = "ForeignOwned")]
    ForeignOwned,
    #[serde(rename = "Subsidiary")]
    Subsidiary,
    #[serde(rename = "TrustOwned")]
    TrustOwned,
    #[serde(rename = "CommunityOwned")]
    CommunityOwned,
    #[serde(rename = "FoundationOwned")]
    FoundationOwned,
    #[serde(rename = "LLC")]
    Llc,
}

impl Default for OwnershipType {
    fn default() -> OwnershipType {
        Self::Private
    }
}
/// What will be the source of incoming funds to your account?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceOfFunds {
    #[serde(rename = "Revenue")]
    Revenue,
    #[serde(rename = "Investments")]
    Investments,
    #[serde(rename = "LoansCredits")]
    LoansCredits,
    #[serde(rename = "TradingIncome")]
    TradingIncome,
    #[serde(rename = "CryptoMining")]
    CryptoMining,
    #[serde(rename = "ClientFunds")]
    ClientFunds,
}

impl Default for SourceOfFunds {
    fn default() -> SourceOfFunds {
        Self::Revenue
    }
}
/// What is the expected frequency of transactions per month?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonthlyTransactionFrequency {
    #[serde(rename = "UpTo10")]
    UpTo10,
    #[serde(rename = "UpTo20")]
    UpTo20,
    #[serde(rename = "UpTo50")]
    UpTo50,
    #[serde(rename = "Above50")]
    Above50,
}

impl Default for MonthlyTransactionFrequency {
    fn default() -> MonthlyTransactionFrequency {
        Self::UpTo10
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
