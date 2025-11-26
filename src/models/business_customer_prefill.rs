use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomerPrefill {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(
        rename = "RegistrationCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_country: Option<String>,
    /// Name of the company.
    #[serde(rename = "CompanyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Registration number of the business.
    #[serde(rename = "RegistrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename = "LegalAddress", skip_serializing_if = "Option::is_none")]
    pub legal_address: Option<Box<models::StreetAddress>>,
    #[serde(rename = "IncorporationDate", skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<String>,
    #[serde(rename = "EntityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<models::EntityEnum>,
    /// Tax ID of the business.
    #[serde(rename = "TaxID", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Primary website of the business.
    #[serde(rename = "PrimaryWebsite", skip_serializing_if = "Option::is_none")]
    pub primary_website: Option<String>,
    /// Trade name of the business (DBA).
    #[serde(rename = "TradeName", skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    /// List of your registered foreign branches, including registered country and any applicable registration numbers
    #[serde(
        rename = "RegisteredForeignBranches",
        skip_serializing_if = "Option::is_none"
    )]
    pub registered_foreign_branches: Option<Vec<models::RegisteredForeignBranchesInput>>,
    #[serde(
        rename = "PrimaryPhysicalAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_physical_address: Option<Box<models::StreetAddress>>,
    /// Ownership type
    #[serde(rename = "OwnershipType", skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<OwnershipType>,
    /// Legal entity identifier (LEI) code of the business.
    #[serde(
        rename = "LegalEntityIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub legal_entity_identifier: Option<String>,
    /// Please provide your NAICS (North American Industry Classification System) code. If you do not have a NAICS code, please select the closest corresponding code that best matches your industry classification - https://www.naics.com/search/
    #[serde(rename = "NAICSCode", skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<String>,
    /// What will be the source of incoming funds to your account?
    #[serde(rename = "SourceOfFunds", skip_serializing_if = "Option::is_none")]
    pub source_of_funds: Option<SourceOfFunds>,
    #[serde(rename = "FinancialsUsd", skip_serializing_if = "Option::is_none")]
    pub financials_usd: Option<Box<models::BusinessFinancialsInput>>,
    /// What is the expected frequency of transactions per month?
    #[serde(
        rename = "MonthlyTransactionFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub monthly_transaction_frequency: Option<MonthlyTransactionFrequency>,
    /// Information about UBOs (25% of more of ownership) and Representatives.
    #[serde(rename = "Associates", skip_serializing_if = "Option::is_none")]
    pub associates: Option<Vec<models::AssociateInformationInput>>,
    /// Information about Corporate Shareholders (25% of more of ownership).
    #[serde(rename = "BusinessAssociates", skip_serializing_if = "Option::is_none")]
    pub business_associates: Option<Vec<models::BusinessAssociateInformationInput>>,
    #[serde(rename = "AMLCTFRegulated", skip_serializing_if = "Option::is_none")]
    pub amlctf_regulated: Option<Box<models::BusinessCustomerPrefillAmlctfRegulated>>,
}

impl BusinessCustomerPrefill {
    pub fn new(r#type: Type) -> BusinessCustomerPrefill {
        BusinessCustomerPrefill {
            r#type,
            registration_country: None,
            company_name: None,
            registration_number: None,
            legal_address: None,
            incorporation_date: None,
            entity_type: None,
            tax_id: None,
            primary_website: None,
            trade_name: None,
            registered_foreign_branches: None,
            primary_physical_address: None,
            ownership_type: None,
            legal_entity_identifier: None,
            naics_code: None,
            source_of_funds: None,
            financials_usd: None,
            monthly_transaction_frequency: None,
            associates: None,
            business_associates: None,
            amlctf_regulated: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BusinessCustomerPrefill")]
    BusinessCustomerPrefill,
}

impl Default for Type {
    fn default() -> Type {
        Self::BusinessCustomerPrefill
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
