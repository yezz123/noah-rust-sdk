use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerFormRequest {
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    /// List of fiat options to be supported by the customer.
    #[serde(rename = "FiatOptions")]
    pub fiat_options: Vec<models::FiatOption>,
    /// Form input to be submitted based on returned FormSchema
    #[serde(rename = "FormInput", skip_serializing_if = "Option::is_none")]
    pub form_input: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl CustomerFormRequest {
    pub fn new(return_url: String, fiat_options: Vec<models::FiatOption>) -> CustomerFormRequest {
        CustomerFormRequest {
            return_url,
            fiat_options,
            form_input: None,
        }
    }
}
