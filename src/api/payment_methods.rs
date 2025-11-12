//! Payment methods API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::common::*;
use crate::models::payment_methods::GetPaymentMethodsResponse;

impl NoahClient {
    /// Get payment methods (async)
    #[cfg(feature = "async")]
    pub async fn get_payment_methods(
        &self,
        customer_id: &CustomerID,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetPaymentMethodsResponse> {
        let mut path = "/payment-methods".to_string();
        let mut query_params = vec![format!("CustomerID={customer_id}")];

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get(&path).await
    }

    /// Get payment methods (blocking)
    #[cfg(feature = "sync")]
    pub fn get_payment_methods_blocking(
        &self,
        customer_id: &CustomerID,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetPaymentMethodsResponse> {
        let mut path = "/payment-methods".to_string();
        let mut query_params = vec![format!("CustomerID={customer_id}")];

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get_blocking(&path)
    }
}
