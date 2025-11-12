//! Configuration for the Noah SDK

use url::Url;

/// Environment configuration
#[derive(Debug, Clone)]
pub enum Environment {
    /// Sandbox environment
    Sandbox,
    /// Production environment
    Production,
    /// Custom base URL
    Custom(Url),
}

impl Environment {
    /// Get the base URL for the environment
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

impl Default for Environment {
    fn default() -> Self {
        Environment::Sandbox
    }
}

/// Client configuration
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
    pub fn new(environment: Environment) -> Self {
        Self {
            base_url: environment.base_url(),
            timeout_secs: 30,
            user_agent: format!("noah-sdk/{}", env!("CARGO_PKG_VERSION")),
            enable_logging: false,
        }
    }

    /// Set the timeout
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set a custom user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Enable logging
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

