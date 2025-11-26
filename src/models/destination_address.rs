use crate::models;
use serde::{Deserialize, Serialize};

/// DestinationAddress : A destination address to transfer cryptocurrency
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationAddress {
    /// Address for transfer
    #[serde(rename = "Address")]
    pub address: String,
}

impl DestinationAddress {
    /// A destination address to transfer cryptocurrency
    pub fn new(address: String) -> DestinationAddress {
        DestinationAddress { address }
    }
}
