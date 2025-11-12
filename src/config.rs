//! Configuration for the Noah SDK
//!
//! This module provides configuration types for the Noah SDK client.
//!
//! # Examples
//!
//! ```no_run
//! use noah_sdk::{Config, Environment};
//!
//! // Use sandbox environment
//! let config = Config::new(Environment::Sandbox);
//!
//! // Use production environment
//! let config = Config::new(Environment::Production);
//!
//! // Use custom URL
//! let custom_url = url::Url::parse("https://api.example.com/v1").unwrap();
//! let config = Config::new(Environment::Custom(custom_url));
//!
//! // Configure timeout and user agent
//! let config = Config::new(Environment::Sandbox)
//!     .with_timeout(60)
//!     .with_user_agent("my-app/1.0".to_string())
//!     .with_logging(true);
//! ```

use url::Url;

/// Environment configuration for the Noah API
///
/// Determines which API endpoint the client will connect to.
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::Environment;
///
/// // Use sandbox for testing
/// let env = Environment::Sandbox;
///
/// // Use production
/// let env = Environment::Production;
///
/// // Use custom endpoint
/// let custom_url = url::Url::parse("https://api.example.com/v1").unwrap();
/// let env = Environment::Custom(custom_url);
/// ```
#[derive(Debug, Clone, Default)]
pub enum Environment {
    /// Sandbox environment
    #[default]
    Sandbox,
    /// Production environment
    Production,
    /// Custom base URL
    Custom(Url),
}

impl Environment {
    /// Get the base URL for the environment
    ///
    /// Returns the base URL that will be used for API requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::Environment;
    ///
    /// let env = Environment::Sandbox;
    /// let url = env.base_url();
    /// println!("Base URL: {}", url);
    /// ```
    pub fn base_url(&self) -> Url {
        match self {
            Environment::Sandbox => {
                Url::parse("https://api.sandbox.noah.com/v1").expect("Invalid sandbox URL")
            }
            Environment::Production => {
                Url::parse("https://api.noah.com/v1").expect("Invalid production URL")
            }
            Environment::Custom(url) => url.clone(),
        }
    }
}

/// Client configuration
///
/// Configuration options for the Noah SDK client, including base URL, timeout,
/// user agent, and logging settings.
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::{Config, Environment};
///
/// // Default configuration
/// let config = Config::new(Environment::Sandbox);
///
/// // Custom configuration
/// let config = Config::new(Environment::Production)
///     .with_timeout(60)
///     .with_user_agent("my-app/1.0".to_string())
///     .with_logging(true);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL for API requests
    pub base_url: Url,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// User agent string
    pub user_agent: String,
    /// Enable request/response logging
    pub enable_logging: bool,
}

impl Config {
    /// Create a new configuration with default values
    ///
    /// Creates a configuration with:
    /// - Base URL from the environment
    /// - 30 second timeout
    /// - Default user agent (includes SDK version)
    /// - Logging disabled
    ///
    /// # Arguments
    ///
    /// * `environment` - The environment to use (Sandbox, Production, or Custom)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Sandbox);
    /// ```
    pub fn new(environment: Environment) -> Self {
        Self {
            base_url: environment.base_url(),
            timeout_secs: 30,
            user_agent: format!("noah-sdk/{}", env!("CARGO_PKG_VERSION")),
            enable_logging: false,
        }
    }

    /// Set the request timeout in seconds
    ///
    /// # Arguments
    ///
    /// * `timeout_secs` - Timeout in seconds
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Sandbox)
    ///     .with_timeout(60); // 60 second timeout
    /// ```
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set a custom user agent string
    ///
    /// # Arguments
    ///
    /// * `user_agent` - Custom user agent string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Sandbox)
    ///     .with_user_agent("my-app/1.0".to_string());
    /// ```
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Enable or disable request/response logging
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable logging
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::{Config, Environment};
    ///
    /// let config = Config::new(Environment::Sandbox)
    ///     .with_logging(true);
    /// ```
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(Environment::default())
    }
}
