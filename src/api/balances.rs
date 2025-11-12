//! Balances API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::balances::GetBalancesResponse;

impl NoahClient {
    /// Get balances (async)
    #[cfg(feature = "async")]
    pub async fn get_balances(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetBalancesResponse> {
        let mut path = "/balances".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get(&path).await
    }

    /// Get balances (blocking)
    #[cfg(feature = "sync")]
    pub fn get_balances_blocking(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetBalancesResponse> {
        let mut path = "/balances".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={}", size));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={}", token));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get_blocking(&path)
    }
}

