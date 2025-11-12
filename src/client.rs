//! Main HTTP client for the Noah SDK
//!
//! This module provides the core HTTP client for making requests to the Noah API.
//! The client supports both async and blocking (synchronous) operations.
//!
//! # Features
//!
//! - **Async Support**: Use `async`/`await` for non-blocking operations
//! - **Blocking Support**: Use synchronous methods for simpler code
//! - **Automatic Authentication**: Handles JWT signing and API key authentication
//! - **Error Handling**: Comprehensive error types with detailed context
//!
//! # Examples
//!
//! ## Async Client
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
//! // Use async methods
//! let balances = client.get_balances(None, None).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking Client
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
//! // Use blocking methods
//! let balances = client.get_balances_blocking(None, None)?;
//! # Ok(())
//! # }
//! ```

use crate::auth::AuthConfig;
use crate::config::Config;
use crate::error::{NoahError, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use url::Url;

/// Main client for interacting with the Noah API
///
/// This client provides methods to interact with all Noah API endpoints.
/// It handles authentication, request building, and response parsing automatically.
///
/// # Thread Safety
///
/// The client is `Clone` and can be shared across threads safely.
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
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct NoahClient {
    config: Config,
    auth_config: AuthConfig,
    #[cfg(feature = "async")]
    client: reqwest::Client,
    #[cfg(feature = "sync")]
    blocking_client: reqwest::blocking::Client,
}

impl NoahClient {
    /// Create a new client with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration (environment, timeout, etc.)
    /// * `auth_config` - Authentication configuration (API key or JWT secret)
    ///
    /// # Returns
    ///
    /// Returns a new `NoahClient` instance ready to make API requests.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP client cannot be created
    /// - The user agent string is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
    ///
    /// # #[cfg(feature = "async")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Using API key authentication
    /// let config = Config::new(Environment::Sandbox);
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// let client = NoahClient::new(config.clone(), auth)?;
    ///
    /// // Using JWT signing
    /// let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
    /// let client = NoahClient::new(config.clone(), auth)?;
    ///
    /// // Using both
    /// let auth = AuthConfig::with_both(
    ///     "your-api-key".to_string(),
    ///     "your-secret-key".to_string()
    /// );
    /// let client = NoahClient::new(config, auth)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: Config, auth_config: AuthConfig) -> Result<Self> {
        #[cfg(feature = "async")]
        let client = {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&config.user_agent)
                    .map_err(|e| NoahError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
            );

            reqwest::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .map_err(NoahError::HttpError)?
        };

        #[cfg(feature = "sync")]
        let blocking_client = {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&config.user_agent)
                    .map_err(|e| NoahError::Other(anyhow::anyhow!("Invalid user agent: {e}")))?,
            );

            reqwest::blocking::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .map_err(|e| {
                    NoahError::Other(anyhow::anyhow!("Failed to create blocking client: {e}"))
                })?
        };

        Ok(Self {
            config,
            auth_config,
            #[cfg(feature = "async")]
            client,
            #[cfg(feature = "sync")]
            blocking_client,
        })
    }

    /// Get the base URL for API requests
    ///
    /// Returns the base URL that all API requests will be made against.
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
    /// let base_url = client.base_url();
    /// println!("API base URL: {}", base_url);
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Build a full URL from a path
    fn build_url(&self, path: &str) -> Result<Url> {
        // If path starts with /, we need to preserve the base URL's path
        let path_to_join = path.strip_prefix('/').unwrap_or(path);

        let mut url = self.config.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| NoahError::Other(anyhow::anyhow!("Cannot be a base URL")))?
            .pop_if_empty()
            .extend(path_to_join.split('/').filter(|s| !s.is_empty()));

        Ok(url)
    }

    #[cfg(feature = "async")]
    /// Make an async GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        let mut builder = self.client.get(url.as_str());

        builder = crate::auth::add_auth_headers_async(
            builder,
            &self.auth_config,
            "GET",
            url.path(),
            None,
        )?;

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async POST request
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let mut builder = self.client.post(url.as_str()).body(body_bytes.clone());

        builder = crate::auth::add_auth_headers_async(
            builder,
            &self.auth_config,
            "POST",
            url.path(),
            Some(&body_bytes),
        )?;

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "async")]
    /// Make an async PUT request
    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let mut builder = self.client.put(url.as_str()).body(body_bytes.clone());

        builder = crate::auth::add_auth_headers_async(
            builder,
            &self.auth_config,
            "PUT",
            url.path(),
            Some(&body_bytes),
        )?;

        let response = builder.send().await?;
        self.handle_response(response).await
    }

    #[cfg(feature = "sync")]
    /// Make a blocking GET request
    pub fn get_blocking<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        let mut builder = self.blocking_client.get(url.as_str());

        builder = crate::auth::add_auth_headers_sync(
            builder,
            &self.auth_config,
            "GET",
            url.path(),
            None,
        )?;

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking POST request
    pub fn post_blocking<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let mut builder = self
            .blocking_client
            .post(url.as_str())
            .body(body_bytes.clone());

        builder = crate::auth::add_auth_headers_sync(
            builder,
            &self.auth_config,
            "POST",
            url.path(),
            Some(&body_bytes),
        )?;

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "sync")]
    /// Make a blocking PUT request
    pub fn put_blocking<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let body_bytes = serde_json::to_vec(body)?;
        let mut builder = self
            .blocking_client
            .put(url.as_str())
            .body(body_bytes.clone());

        builder = crate::auth::add_auth_headers_sync(
            builder,
            &self.auth_config,
            "PUT",
            url.path(),
            Some(&body_bytes),
        )?;

        let response = builder.send()?;
        self.handle_blocking_response(response)
    }

    #[cfg(feature = "async")]
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let url = response.url().clone();
        let text = response.text().await?;

        if status.is_success() {
            if text.is_empty() {
                // Handle 204 No Content
                serde_json::from_str("null")
                    .map_err(|_| NoahError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(NoahError::DeserializationError)
            }
        } else {
            // Try to parse as error response
            match serde_json::from_str::<crate::error::ApiErrorResponse>(&text) {
                Ok(api_error) => Err(NoahError::ApiError(Box::new(api_error))),
                Err(_) => Err(NoahError::Other(anyhow::anyhow!(
                    "HTTP {} from {}: {}",
                    status,
                    url,
                    if text.len() > 200 {
                        format!("{}...", &text[..200])
                    } else {
                        text
                    }
                ))),
            }
        }
    }

    #[cfg(feature = "sync")]
    fn handle_blocking_response<T: DeserializeOwned>(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<T> {
        let status = response.status();
        let text = response.text()?;

        if status.is_success() {
            if text.is_empty() {
                // Handle 204 No Content
                serde_json::from_str("null")
                    .map_err(|_| NoahError::ValidationError("Empty response body".to_string()))
            } else {
                serde_json::from_str(&text).map_err(NoahError::DeserializationError)
            }
        } else {
            // Try to parse as error response
            match serde_json::from_str::<crate::error::ApiErrorResponse>(&text) {
                Ok(api_error) => Err(NoahError::ApiError(Box::new(api_error))),
                Err(_) => Err(NoahError::Other(anyhow::anyhow!("HTTP {status}: {text}"))),
            }
        }
    }
}
