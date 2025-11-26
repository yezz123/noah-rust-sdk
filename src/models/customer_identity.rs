use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerIdentity {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: String,
    #[serde(rename = "IDNumber")]
    pub id_number: String,
    #[serde(rename = "IssuedDate", skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<String>,
    #[serde(rename = "ExpiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// Type of identification document:  * DrivingLicense  * NationalIDCard  * Passport  * AddressProof  * ResidencePermit  * TaxID
    #[serde(rename = "IDType")]
    pub id_type: String,
}

impl CustomerIdentity {
    pub fn new(issuing_country: String, id_number: String, id_type: String) -> CustomerIdentity {
        CustomerIdentity {
            issuing_country,
            id_number,
            issued_date: None,
            expiry_date: None,
            id_type,
        }
    }
}
