use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPaymentMethodCardDisplay {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "Last4")]
    pub last4: String,
    /// The card scheme:  * Mastercard  * Visa
    #[serde(rename = "Scheme")]
    pub scheme: String,
}

impl FiatPaymentMethodCardDisplay {
    pub fn new(r#type: Type, last4: String, scheme: String) -> FiatPaymentMethodCardDisplay {
        FiatPaymentMethodCardDisplay {
            r#type,
            last4,
            scheme,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FiatPaymentMethodCardDisplay")]
    FiatPaymentMethodCardDisplay,
}

impl Default for Type {
    fn default() -> Type {
        Self::FiatPaymentMethodCardDisplay
    }
}
