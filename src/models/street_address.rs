use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreetAddress {
    /// Street: the primary name of an address's street.
    #[serde(rename = "Street")]
    pub street: String,
    /// Street2: the secondary name of an address's street.
    #[serde(rename = "Street2", skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    /// City: name of an address's city or town.
    #[serde(rename = "City")]
    pub city: String,
    /// PostCode: the address's postcode
    #[serde(rename = "PostCode")]
    pub post_code: String,
    /// State: the address's state / province / county. For USA and Canada, state code in ISO 3166-2 code (e.g. CA) is required.
    #[serde(rename = "State")]
    pub state: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "Country")]
    pub country: String,
}

impl StreetAddress {
    pub fn new(
        street: String,
        city: String,
        post_code: String,
        state: String,
        country: String,
    ) -> StreetAddress {
        StreetAddress {
            street,
            street2: None,
            city,
            post_code,
            state,
            country,
        }
    }
}
