//! Customer-related models

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Full name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullName {
    /// First name
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// Last name
    #[serde(rename = "LastName")]
    pub last_name: String,
    /// Middle name (optional)
    #[serde(rename = "MiddleName")]
    pub middle_name: Option<String>,
}

/// Street address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreetAddress {
    /// Street
    #[serde(rename = "Street")]
    pub street: String,
    /// Street 2 (optional)
    #[serde(rename = "Street2")]
    pub street2: Option<String>,
    /// City
    #[serde(rename = "City")]
    pub city: String,
    /// Post code
    #[serde(rename = "PostCode")]
    pub post_code: String,
    /// State
    #[serde(rename = "State")]
    pub state: String,
    /// Country
    #[serde(rename = "Country")]
    pub country: CountryCode,
}

/// ID type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum IDType {
    /// Driving license
    DrivingLicense,
    /// National ID card
    NationalIdCard,
    /// Passport
    Passport,
    /// Address proof
    AddressProof,
    /// Residence permit
    ResidencePermit,
    /// Tax ID
    TaxId,
}

/// Customer identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerIdentity {
    /// Issuing country
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: CountryCode,
    /// ID number
    #[serde(rename = "IDNumber")]
    pub id_number: String,
    /// Issued date (optional)
    #[serde(rename = "IssuedDate")]
    pub issued_date: Option<Date>,
    /// Expiry date (optional)
    #[serde(rename = "ExpiryDate")]
    pub expiry_date: Option<Date>,
    /// ID type
    #[serde(rename = "IDType")]
    pub id_type: IDType,
}

/// Verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum VerificationStatus {
    /// Approved
    Approved,
    /// Pending
    Pending,
    /// Declined
    Declined,
}

/// Verification (deprecated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    /// Model
    #[serde(rename = "Model")]
    pub model: String,
    /// Status
    #[serde(rename = "Status")]
    pub status: VerificationStatus,
}

/// Verifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verifications {
    /// Type
    #[serde(rename = "Type")]
    pub verification_type: String,
    /// Status
    #[serde(rename = "Status")]
    pub status: String,
    /// Entity verifications
    #[serde(rename = "EntityVerifications")]
    pub entity_verifications: Vec<serde_json::Value>,
}

/// Customer (discriminated union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Customer {
    /// Individual customer
    #[serde(rename = "Individual")]
    Individual(IndividualCustomer),
    /// Business customer
    #[serde(rename = "Business")]
    Business(BusinessCustomer),
}

/// Individual customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualCustomer {
    /// Type
    #[serde(rename = "Type")]
    pub customer_type: String,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// Created timestamp
    #[serde(rename = "Created")]
    pub created: DateTime,
    /// Date of birth
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: Date,
    /// Full name
    #[serde(rename = "FullName")]
    pub full_name: FullName,
    /// Identities
    #[serde(rename = "Identities")]
    pub identities: Vec<CustomerIdentity>,
    /// Primary residence
    #[serde(rename = "PrimaryResidence")]
    pub primary_residence: StreetAddress,
    /// Verification (deprecated)
    #[serde(rename = "Verification")]
    pub verification: Verification,
    /// Verifications
    #[serde(rename = "Verifications")]
    pub verifications: Option<Verifications>,
    /// Metadata
    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Business customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCustomer {
    /// Type
    #[serde(rename = "Type")]
    pub customer_type: String,
    /// Customer ID
    #[serde(rename = "CustomerID")]
    pub customer_id: CustomerID,
    /// Created timestamp
    #[serde(rename = "Created")]
    pub created: DateTime,
    /// Registered name
    #[serde(rename = "RegisteredName")]
    pub registered_name: String,
    /// Email (optional)
    #[serde(rename = "Email")]
    pub email: Option<String>,
    /// Registration number (optional)
    #[serde(rename = "RegistrationNumber")]
    pub registration_number: Option<String>,
    /// Registration country (optional)
    #[serde(rename = "RegistrationCountry")]
    pub registration_country: Option<CountryCode>,
    /// Registered address (optional)
    #[serde(rename = "RegisteredAddress")]
    pub registered_address: Option<StreetAddress>,
    /// Incorporation date (optional)
    #[serde(rename = "IncorporationDate")]
    pub incorporation_date: Option<Date>,
    /// Verification (deprecated)
    #[serde(rename = "Verification")]
    pub verification: Verification,
    /// Verifications
    #[serde(rename = "Verifications")]
    pub verifications: Option<Verifications>,
    /// Metadata
    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Individual customer input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualCustomerInput {
    /// Type
    #[serde(rename = "Type")]
    pub customer_type: String,
    /// Full name
    #[serde(rename = "FullName")]
    pub full_name: FullName,
    /// Date of birth
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: Date,
    /// Email (optional)
    #[serde(rename = "Email")]
    pub email: Option<String>,
    /// Phone number (optional)
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Option<PhoneNumber>,
    /// Identities
    #[serde(rename = "Identities")]
    pub identities: Vec<CustomerIdentity>,
    /// Primary residence
    #[serde(rename = "PrimaryResidence")]
    pub primary_residence: StreetAddress,
}

/// Business customer input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCustomerInput {
    /// Type
    #[serde(rename = "Type")]
    pub customer_type: String,
    /// Registered name
    #[serde(rename = "RegisteredName")]
    pub registered_name: String,
    /// Email (optional)
    #[serde(rename = "Email")]
    pub email: Option<String>,
    /// Registration number
    #[serde(rename = "RegistrationNumber")]
    pub registration_number: String,
    /// Registration country
    #[serde(rename = "RegistrationCountry")]
    pub registration_country: CountryCode,
    /// Registered address
    #[serde(rename = "RegisteredAddress")]
    pub registered_address: StreetAddress,
    /// Incorporation date
    #[serde(rename = "IncorporationDate")]
    pub incorporation_date: Date,
}

/// Customer input (discriminated union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum CustomerInput {
    /// Individual customer input
    #[serde(rename = "Individual")]
    Individual(IndividualCustomerInput),
    /// Business customer input
    #[serde(rename = "Business")]
    Business(BusinessCustomerInput),
}

/// Get customers response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomersResponse {
    /// List of customers
    #[serde(rename = "Items")]
    pub items: Vec<Customer>,
    /// Pagination token
    #[serde(rename = "PageToken")]
    pub page_token: Option<String>,
}
