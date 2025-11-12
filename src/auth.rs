//! Authentication module for JWT signing and API key management
//!
//! This module provides authentication functionality for the Noah SDK.
//! It supports both API key authentication and JWT signing.
//!
//! # Authentication Methods
//!
//! ## API Key Authentication
//!
//! Simple authentication using an API key:
//!
//! ```no_run
//! use noah_sdk::AuthConfig;
//!
//! let auth = AuthConfig::with_api_key("your-api-key".to_string());
//! ```
//!
//! ## JWT Signing
//!
//! More secure authentication using JWT signing with a secret key:
//!
//! ```no_run
//! use noah_sdk::AuthConfig;
//!
//! let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
//! ```
//!
//! ## Both Methods
//!
//! You can use both API key and JWT signing together:
//!
//! ```no_run
//! use noah_sdk::AuthConfig;
//!
//! let auth = AuthConfig::with_both(
//!     "your-api-key".to_string(),
//!     "your-secret-key".to_string()
//! );
//! ```

use crate::error::{NoahError, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Authentication configuration
///
/// Configures how the client authenticates with the Noah API.
/// Supports API key authentication, JWT signing, or both.
///
/// # Examples
///
/// ```no_run
/// use noah_sdk::AuthConfig;
///
/// // API key only
/// let auth = AuthConfig::with_api_key("your-api-key".to_string());
///
/// // JWT signing only
/// let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
///
/// // Both methods
/// let auth = AuthConfig::with_both(
///     "your-api-key".to_string(),
///     "your-secret-key".to_string()
/// );
/// ```
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// API key for X-Api-Key header
    pub api_key: Option<String>,
    /// Secret key for JWT signing
    pub secret_key: Option<String>,
}

impl AuthConfig {
    /// Create new auth config with API key only
    ///
    /// This method uses simple API key authentication via the `X-Api-Key` header.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Noah API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::AuthConfig;
    ///
    /// let auth = AuthConfig::with_api_key("your-api-key".to_string());
    /// ```
    pub fn with_api_key(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
            secret_key: None,
        }
    }

    /// Create new auth config with JWT secret key only
    ///
    /// This method uses JWT signing for authentication. The client will automatically
    /// generate and sign JWTs for each request using the `Api-Signature` header.
    ///
    /// # Arguments
    ///
    /// * `secret_key` - Your Noah secret key for JWT signing
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::AuthConfig;
    ///
    /// let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
    /// ```
    pub fn with_secret_key(secret_key: String) -> Self {
        Self {
            api_key: None,
            secret_key: Some(secret_key),
        }
    }

    /// Create new auth config with both API key and secret key
    ///
    /// This method uses both API key authentication and JWT signing.
    /// Both headers will be included in requests.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Noah API key
    /// * `secret_key` - Your Noah secret key for JWT signing
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use noah_sdk::AuthConfig;
    ///
    /// let auth = AuthConfig::with_both(
    ///     "your-api-key".to_string(),
    ///     "your-secret-key".to_string()
    /// );
    /// ```
    pub fn with_both(api_key: String, secret_key: String) -> Self {
        Self {
            api_key: Some(api_key),
            secret_key: Some(secret_key),
        }
    }
}

/// JWT payload structure
#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    /// Issued at timestamp
    iat: u64,
    /// Expiration timestamp
    exp: u64,
    /// HTTP method
    method: String,
    /// Request path
    path: String,
    /// Request body hash (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    body_hash: Option<String>,
}

/// Generate JWT signature for Api-Signature header
pub fn generate_jwt_signature(
    secret_key: &str,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| NoahError::JwtError(format!("Failed to get timestamp: {e}")))?;

    let iat = now.as_secs();
    let exp = iat + 300; // 5 minutes expiration

    // Calculate body hash if body is provided
    let body_hash = body.map(|b| {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(b);
        format!("{:x}", hasher.finalize())
    });

    let payload = JwtPayload {
        iat,
        exp,
        method: method.to_uppercase(),
        path: path.to_string(),
        body_hash,
    };

    // Create JWT header
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "JWT"
    });

    // Encode header and payload
    let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header)?);
    let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload)?);
    let message = format!("{header_b64}.{payload_b64}");

    // Sign with HMAC-SHA256
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| NoahError::JwtError(format!("Invalid secret key: {e}")))?;
    mac.update(message.as_bytes());
    let signature = mac.finalize();
    let signature_b64 = URL_SAFE_NO_PAD.encode(signature.into_bytes());

    Ok(format!("{message}.{signature_b64}"))
}

/// Add authentication headers to a request builder
#[cfg(feature = "async")]
pub fn add_auth_headers_async(
    builder: reqwest::RequestBuilder,
    auth_config: &AuthConfig,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<reqwest::RequestBuilder> {
    let mut builder = builder;

    // Add API key if available
    if let Some(ref api_key) = auth_config.api_key {
        builder = builder.header("X-Api-Key", api_key);
    }

    // Add JWT signature if secret key is available
    if let Some(ref secret_key) = auth_config.secret_key {
        let signature = generate_jwt_signature(secret_key, method, path, body)?;
        builder = builder.header("Api-Signature", signature);
    }

    Ok(builder)
}

/// Add authentication headers to a request builder (blocking)
#[cfg(feature = "sync")]
pub fn add_auth_headers_sync(
    builder: reqwest::blocking::RequestBuilder,
    auth_config: &AuthConfig,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<reqwest::blocking::RequestBuilder> {
    let mut builder = builder;

    // Add API key if available
    if let Some(ref api_key) = auth_config.api_key {
        builder = builder.header("X-Api-Key", api_key);
    }

    // Add JWT signature if secret key is available
    if let Some(ref secret_key) = auth_config.secret_key {
        let signature = generate_jwt_signature(secret_key, method, path, body)?;
        builder = builder.header("Api-Signature", signature);
    }

    Ok(builder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_signature_generation() {
        let secret = "test-secret-key";
        let method = "GET";
        let path = "/balances";
        let signature = generate_jwt_signature(secret, method, path, None).unwrap();
        assert!(!signature.is_empty());
        assert!(signature.contains('.'));
    }
}
