//! Customers API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::common::*;
use crate::models::customers::{Customer, CustomerInput, GetCustomersResponse};

impl NoahClient {
    /// Get customer by ID (async)
    #[cfg(feature = "async")]
    pub async fn get_customer(&self, customer_id: &CustomerID) -> Result<Customer> {
        let path = format!("/customers/{}", customer_id);
        self.get(&path).await
    }

    /// Get customer by ID (blocking)
    #[cfg(feature = "sync")]
    pub fn get_customer_blocking(&self, customer_id: &CustomerID) -> Result<Customer> {
        let path = format!("/customers/{}", customer_id);
        self.get_blocking(&path)
    }

    /// Create or update customer (async)
    #[cfg(feature = "async")]
    pub async fn create_or_update_customer(
        &self,
        customer_id: &CustomerID,
        customer: &CustomerInput,
    ) -> Result<()> {
        let path = format!("/customers/{}", customer_id);
        let _response: Option<serde_json::Value> = self.put(&path, customer).await?;
        Ok(())
    }

    /// Create or update customer (blocking)
    #[cfg(feature = "sync")]
    pub fn create_or_update_customer_blocking(
        &self,
        customer_id: &CustomerID,
        customer: &CustomerInput,
    ) -> Result<()> {
        let path = format!("/customers/{}", customer_id);
        let _response: Option<serde_json::Value> = self.put_blocking(&path, customer)?;
        Ok(())
    }

    /// Get customers (async)
    #[cfg(feature = "async")]
    pub async fn get_customers(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        sort_direction: Option<&SortDirection>,
    ) -> Result<GetCustomersResponse> {
        let mut path = "/customers".to_string();
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

    /// Get customers (blocking)
    #[cfg(feature = "sync")]
    pub fn get_customers_blocking(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        sort_direction: Option<&SortDirection>,
    ) -> Result<GetCustomersResponse> {
        let mut path = "/customers".to_string();
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
}

