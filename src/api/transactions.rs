//! Transactions API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::common::*;
use crate::models::transactions::{
    GetTransactionsResponse, PrepareSellRequest, PrepareSellResponse, SellRequest, SellResponse,
    Transaction,
};

impl NoahClient {
    /// Get transactions (async)
    #[cfg(feature = "async")]
    pub async fn get_transactions(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        sort_direction: Option<&SortDirection>,
    ) -> Result<GetTransactionsResponse> {
        let mut path = "/transactions".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }
        if let Some(sort) = sort_direction {
            query_params.push(format!("SortDirection={:?}", sort));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get(&path).await
    }

    /// Get transactions (blocking)
    #[cfg(feature = "sync")]
    pub fn get_transactions_blocking(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        sort_direction: Option<&SortDirection>,
    ) -> Result<GetTransactionsResponse> {
        let mut path = "/transactions".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }
        if let Some(sort) = sort_direction {
            query_params.push(format!("SortDirection={:?}", sort));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get_blocking(&path)
    }

    /// Get transaction by ID (async)
    #[cfg(feature = "async")]
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        let path = format!("/transactions/{}", transaction_id);
        self.get(&path).await
    }

    /// Get transaction by ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_transaction_blocking(&self, transaction_id: &str) -> Result<Transaction> {
        let path = format!("/transactions/{}", transaction_id);
        self.get_blocking(&path)
    }

    /// Prepare sell transaction (async)
    #[cfg(feature = "async")]
    pub async fn prepare_sell(
        &self,
        request: &PrepareSellRequest,
    ) -> Result<PrepareSellResponse> {
        self.post("/transactions/sell/prepare", request).await
    }

    /// Prepare sell transaction (blocking)
    #[cfg(feature = "sync")]
    pub fn prepare_sell_blocking(&self, request: &PrepareSellRequest) -> Result<PrepareSellResponse> {
        self.post_blocking("/transactions/sell/prepare", request)
    }

    /// Create sell transaction (async)
    #[cfg(feature = "async")]
    pub async fn sell(&self, request: &SellRequest) -> Result<SellResponse> {
        self.post("/transactions/sell", request).await
    }

    /// Create sell transaction (blocking)
    #[cfg(feature = "sync")]
    pub fn sell_blocking(&self, request: &SellRequest) -> Result<SellResponse> {
        self.post_blocking("/transactions/sell", request)
    }
}

