//! Balances API
//!
//! This module provides functionality to retrieve account balances from the Noah API.
//!
//! # Examples
//!
//! ## Async Example
//!
//! ```no_run
//! use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
//!
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Sandbox);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = NoahClient::new(config, auth)?;
//!
//! // Get first page of balances
//! let balances = client.get_balances(None, None).await?;
//! println!("Balances: {:?}", balances);
//!
//! // Get next page if available
//! if let Some(ref next_token) = balances.page_token {
//!     let next_page = client.get_balances(Some(20), Some(next_token)).await?;
//!     println!("Next page: {:?}", next_page);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking Example
//!
//! ```no_run
//! use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
//!
//! # #[cfg(feature = "sync")]
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Sandbox);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = NoahClient::new(config, auth)?;
//!
//! // Get balances with pagination
//! let balances = client.get_balances_blocking(Some(50), None)?;
//! println!("Balances: {:?}", balances);
//! # Ok(())
//! # }
//! ```

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::balances::GetBalancesResponse;

impl NoahClient {
    /// Get account balances with optional pagination
    ///
    /// Retrieves a list of balances for the authenticated account. Supports pagination
    /// through `page_size` and `page_token` parameters.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Optional number of items per page (defaults to API default if None)
    /// * `page_token` - Optional token for pagination to get the next page of results
    ///
    /// # Returns
    ///
    /// Returns a [`GetBalancesResponse`] containing the list of balances and pagination information.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The API request fails
    /// - Authentication fails
    /// - The response cannot be deserialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// // Get first page
    /// let balances = client.get_balances(None, None).await?;
    ///
    /// // Get specific page size
    /// let balances = client.get_balances(Some(100), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_balances(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetBalancesResponse> {
        let mut path = "/balances".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get(&path).await
    }

    /// Get account balances with optional pagination (blocking)
    ///
    /// Synchronous version of [`get_balances`](Self::get_balances). See that method for
    /// detailed documentation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    ///
    /// # #[cfg(feature = "sync")]
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// let balances = client.get_balances_blocking(Some(50), None)?;
    /// println!("Balances: {:?}", balances);
    /// ```
    #[cfg(feature = "sync")]
    pub fn get_balances_blocking(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<GetBalancesResponse> {
        let mut path = "/balances".to_string();
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get_blocking(&path)
    }
}
