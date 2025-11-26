use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPaymentMethodIdentifierDisplay {
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Identifier type:  * PhoneNumber  * Email  * TaxID
    #[serde(rename = "IdentifierType")]
    pub identifier_type: String,
    #[serde(rename = "Identifier")]
    pub identifier: String,
}

impl FiatPaymentMethodIdentifierDisplay {
    pub fn new(
        r#type: Type,
        identifier_type: String,
        identifier: String,
    ) -> FiatPaymentMethodIdentifierDisplay {
        FiatPaymentMethodIdentifierDisplay {
            r#type,
            identifier_type,
            identifier,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FiatPaymentMethodIdentifierDisplay")]
    FiatPaymentMethodIdentifierDisplay,
}

impl Default for Type {
    fn default() -> Type {
        Self::FiatPaymentMethodIdentifierDisplay
    }
}
