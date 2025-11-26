use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPaymentMethodBankDisplay {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "AccountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "BankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
}

impl FiatPaymentMethodBankDisplay {
    pub fn new(r#type: Type) -> FiatPaymentMethodBankDisplay {
        FiatPaymentMethodBankDisplay {
            r#type,
            account_number: None,
            bank_code: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FiatPaymentMethodBankDisplay")]
    FiatPaymentMethodBankDisplay,
}

impl Default for Type {
    fn default() -> Type {
        Self::FiatPaymentMethodBankDisplay
    }
}
