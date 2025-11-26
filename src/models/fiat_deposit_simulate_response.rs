use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatDepositSimulateResponse {
    #[serde(rename = "FiatDepositID")]
    pub fiat_deposit_id: String,
}

impl FiatDepositSimulateResponse {
    pub fn new(fiat_deposit_id: String) -> FiatDepositSimulateResponse {
        FiatDepositSimulateResponse { fiat_deposit_id }
    }
}
