use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostedOnboardingRequest {
    /// Custom user defined key value pairs used for storing additional information.
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    /// List of fiat options to be supported by the customer.
    #[serde(rename = "FiatOptions")]
    pub fiat_options: Vec<models::FiatOption>,
    /// Form input to be submitted based on returned FormSchema
    #[serde(rename = "Form", skip_serializing_if = "Option::is_none")]
    pub form: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl HostedOnboardingRequest {
    pub fn new(
        return_url: String,
        fiat_options: Vec<models::FiatOption>,
    ) -> HostedOnboardingRequest {
        HostedOnboardingRequest {
            metadata: None,
            return_url,
            fiat_options,
            form: None,
        }
    }
}
