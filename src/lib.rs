//! # Noah SDK
//!
//! A modern, type-safe Rust SDK for the Noah Business API.
//!
//! ## Features
//!
//! - **Async and Sync Support**: Use async/await or blocking operations
//! - **Type Safety**: Strongly typed models generated from OpenAPI schema
//! - **Dual Authentication**: Support for both JWT signing and API key authentication
//! - **Comprehensive Error Handling**: Detailed error types with context
//!
//! ## Quick Start
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
//! let balances = client.get_balances(None, None).await?;
//! println!("Balances: {:?}", balances);
//! # Ok(())
//! # }
//! ```
//!
//! ## Documentation
//!
//! See the [documentation](https://docs.rs/noah-sdk) for detailed API reference.

pub mod api;
pub mod auth;
#[cfg(feature = "axum")]
pub mod axum_wrappers;

pub mod client;
pub mod config;
pub mod error;
pub mod models;

pub use auth::AuthConfig;
pub use client::NoahClient;
pub use config::{Config, Environment};
pub use error::{NoahError, Result};

// Re-export API modules
pub use api::{
    balances, channels, checkout, customers, onboarding, payment_methods, transactions, workflows,
};
