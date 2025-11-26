use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntercomIdentifyHashResponse {
    /// Intercom identify hash for the user.
    #[serde(rename = "UserHash")]
    pub user_hash: String,
}

impl IntercomIdentifyHashResponse {
    pub fn new(user_hash: String) -> IntercomIdentifyHashResponse {
        IntercomIdentifyHashResponse { user_hash }
    }
}
