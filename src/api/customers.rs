//! Customers API
//!
//! This module provides functionality to manage customers in the Noah API.
//!
//! # Examples
//!
//! ```no_run
//! use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
//! use noah_sdk::models::customers::CustomerInput;
//! use noah_sdk::models::common::CustomerID;
//! use noah_sdk::models::FullName;
//! use noah_sdk::models::StreetAddress;
//!
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::new(Environment::Sandbox);
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! let client = NoahClient::new(config, auth)?;
//!
//! // Create or update a customer
//! use noah_sdk::models::customers::{CustomerInput, IndividualCustomerInput, FullName, StreetAddress};
//! use noah_sdk::models::common::*;
//! let customer_id = "customer-123".to_string();
//! // Note: IndividualCustomerInput requires many fields (customer_type, full_name,
//! // date_of_birth, email, identities, primary_residence, etc.)
//! // This is a simplified example - see the API documentation for full field details
//! # let customer_input = CustomerInput::Individual(IndividualCustomerInput {
//! #     customer_type: "Individual".to_string(),
//! #     full_name: FullName {
//! #         first_name: "John".to_string(),
//! #         last_name: "Doe".to_string(),
//! #         middle_name: None,
//! #     },
//! #     date_of_birth: "1990-01-01".to_string(),
//! #     email: Some("john@example.com".to_string()),
//! #     phone_number: None,
//! #     identities: vec![],
//! #     primary_residence: StreetAddress {
//! #         street: "123 Main St".to_string(),
//! #         street2: None,
//! #         city: "New York".to_string(),
//! #         post_code: "10001".to_string(),
//! #         state: "NY".to_string(),
//! #         country: "US".to_string(),
//! #     },
//! # });
//! client.create_or_update_customer(&customer_id, &customer_input).await?;
//!
//! // Retrieve customer
//! let customer = client.get_customer(&customer_id).await?;
//! println!("Customer: {:?}", customer);
//! # Ok(())
//! # }
//! ```

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::common::*;
use crate::models::customers::{Customer, CustomerInput, GetCustomersResponse};

impl NoahClient {
    /// Retrieve a customer by their ID
    ///
    /// # Arguments
    ///
    /// * `customer_id` - The unique identifier of the customer to retrieve
    ///
    /// # Returns
    ///
    /// Returns a [`Customer`] object containing the customer's information.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The customer ID is invalid
    /// - The customer does not exist
    /// - The API request fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::models::common::CustomerID;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// let customer_id = "customer-123".to_string();
    /// let customer = client.get_customer(&customer_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_customer(&self, customer_id: &CustomerID) -> Result<Customer> {
        let path = format!("/customers/{customer_id}");
        self.get(&path).await
    }

    /// Retrieve a customer by their ID (blocking)
    ///
    /// Synchronous version of [`get_customer`](Self::get_customer). See that method for
    /// detailed documentation.
    #[cfg(feature = "sync")]
    pub fn get_customer_blocking(&self, customer_id: &CustomerID) -> Result<Customer> {
        let path = format!("/customers/{customer_id}");
        self.get_blocking(&path)
    }

    /// Create a new customer or update an existing one
    ///
    /// This method will create a customer if they don't exist, or update an existing customer
    /// if they do. The customer ID is used as the unique identifier.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - The unique identifier for the customer
    /// * `customer` - The customer data to create or update
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation succeeds.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The customer data is invalid
    /// - The API request fails
    /// - Authentication fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::models::customers::CustomerInput;
    /// use noah_sdk::models::common::CustomerID;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// use noah_sdk::models::customers::{CustomerInput, IndividualCustomerInput, FullName, StreetAddress};
    /// use noah_sdk::models::common::*;
    /// let customer_id = "customer-123".to_string();
    /// // Note: IndividualCustomerInput requires many fields - see API docs for details
    /// # let customer = CustomerInput::Individual(IndividualCustomerInput {
    /// #     customer_type: "Individual".to_string(),
    /// #     full_name: FullName {
    /// #         first_name: "John".to_string(),
    /// #         last_name: "Doe".to_string(),
    /// #         middle_name: None,
    /// #     },
    /// #     date_of_birth: "1990-01-01".to_string(),
    /// #     email: Some("john@example.com".to_string()),
    /// #     phone_number: None,
    /// #     identities: vec![],
    /// #     primary_residence: StreetAddress {
    /// #         street: "123 Main St".to_string(),
    /// #         street2: None,
    /// #         city: "New York".to_string(),
    /// #         post_code: "10001".to_string(),
    /// #         state: "NY".to_string(),
    /// #         country: "US".to_string(),
    /// #     },
    /// # });
    /// client.create_or_update_customer(&customer_id, &customer).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn create_or_update_customer(
        &self,
        customer_id: &CustomerID,
        customer: &CustomerInput,
    ) -> Result<()> {
        let path = format!("/customers/{customer_id}");
        let _response: Option<serde_json::Value> = self.put(&path, customer).await?;
        Ok(())
    }

    /// Create a new customer or update an existing one (blocking)
    ///
    /// Synchronous version of [`create_or_update_customer`](Self::create_or_update_customer).
    /// See that method for detailed documentation.
    #[cfg(feature = "sync")]
    pub fn create_or_update_customer_blocking(
        &self,
        customer_id: &CustomerID,
        customer: &CustomerInput,
    ) -> Result<()> {
        let path = format!("/customers/{customer_id}");
        let _response: Option<serde_json::Value> = self.put_blocking(&path, customer)?;
        Ok(())
    }

    /// List customers with optional pagination and sorting
    ///
    /// Retrieves a paginated list of customers. Supports pagination and sorting options.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Optional number of items per page
    /// * `page_token` - Optional token for pagination to get the next page
    /// * `sort_direction` - Optional sort direction (ascending or descending)
    ///
    /// # Returns
    ///
    /// Returns a [`GetCustomersResponse`] containing the list of customers and pagination info.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    /// use noah_sdk::models::common::SortDirection;
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config, auth)?;
    ///
    /// // Get first page, sorted descending
    /// let customers = client.get_customers(
    ///     Some(50),
    ///     None,
    ///     Some(&SortDirection::Desc)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }
        if let Some(sort) = sort_direction {
            query_params.push(format!("SortDirection={sort:?}"));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get(&path).await
    }

    /// List customers with optional pagination and sorting (blocking)
    ///
    /// Synchronous version of [`get_customers`](Self::get_customers). See that method for
    /// detailed documentation.
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
            query_params.push(format!("PageSize={size}"));
        }
        if let Some(token) = page_token {
            query_params.push(format!("PageToken={token}"));
        }
        if let Some(sort) = sort_direction {
            query_params.push(format!("SortDirection={sort:?}"));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.get_blocking(&path)
    }
}
