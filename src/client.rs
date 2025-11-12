//! Main HTTP client for the Noah SDK

use crate::auth::AuthConfig;
use crate::config::Config;
use crate::error::{NoahError, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use url::Url;

/// Main client for interacting with the Noah API
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
    pub fn new(config: Config, auth_config: AuthConfig) -> Result<Self> {
        #[cfg(feature = "async")]
        let client = {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&config.user_agent)
                    .map_err(|e| NoahError::Other(anyhow::anyhow!("Invalid user agent: {}", e)))?,
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
                    .map_err(|e| NoahError::Other(anyhow::anyhow!("Invalid user agent: {}", e)))?,
            );

            reqwest::blocking::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()
                .map_err(|e| NoahError::Other(anyhow::anyhow!("Failed to create blocking client: {}", e)))?
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

    /// Get the base URL
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Build a full URL from a path
    fn build_url(&self, path: &str) -> Result<Url> {
        // If path starts with /, we need to preserve the base URL's path
        let path_to_join = if path.starts_with('/') {
            // Remove leading / and join, then manually add it back
            &path[1..]
        } else {
            path
        };
        
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
        let mut builder = self.blocking_client.post(url.as_str()).body(body_bytes.clone());

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
        let mut builder = self.blocking_client.put(url.as_str()).body(body_bytes.clone());

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
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
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
                Ok(api_error) => Err(NoahError::ApiError(api_error)),
                Err(_) => Err(NoahError::Other(anyhow::anyhow!(
                    "HTTP {} from {}: {}",
                    status,
                    url,
                    if text.len() > 200 { format!("{}...", &text[..200]) } else { text }
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
                Ok(api_error) => Err(NoahError::ApiError(api_error)),
                Err(_) => Err(NoahError::Other(anyhow::anyhow!(
                    "HTTP {}: {}",
                    status,
                    text
                ))),
            }
        }
    }
}

