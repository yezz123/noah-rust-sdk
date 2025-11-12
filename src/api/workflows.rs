//! Workflows API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::workflows::{
    BankDepositToOnchainAddressRequest, BankDepositToOnchainAddressResponse,
};

impl NoahClient {
    /// Create bank deposit to onchain address workflow (async)
    #[cfg(feature = "async")]
    pub async fn create_bank_deposit_to_onchain_address_workflow(
        &self,
        request: &BankDepositToOnchainAddressRequest,
    ) -> Result<BankDepositToOnchainAddressResponse> {
        self.post("/workflows/bank-deposit-to-onchain-address", request)
            .await
    }

    /// Create bank deposit to onchain address workflow (blocking)
    #[cfg(feature = "sync")]
    pub fn create_bank_deposit_to_onchain_address_workflow_blocking(
        &self,
        request: &BankDepositToOnchainAddressRequest,
    ) -> Result<BankDepositToOnchainAddressResponse> {
        self.post_blocking("/workflows/bank-deposit-to-onchain-address", request)
    }
}

