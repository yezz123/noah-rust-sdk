//! Integration tests for the client

use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::utilities_api;

#[tokio::test]
#[ignore] // Requires actual API credentials
async fn test_client_creation() {
    let config = Configuration {
        base_path: "https://api.sandbox.noah.com/v1".to_string(),
        api_key: Some(ApiKey {
            prefix: None,
            key: "test-api-key".to_string(),
        }),
        ..Default::default()
    };
    // Configuration is valid if it can be created
    assert_eq!(config.base_path, "https://api.sandbox.noah.com/v1");
}

#[tokio::test]
#[ignore] // Requires actual API credentials
async fn test_get_balances() {
    let config = Configuration {
        base_path: "https://api.sandbox.noah.com/v1".to_string(),
        api_key: Some(ApiKey {
            prefix: None,
            key: "test-api-key".to_string(),
        }),
        ..Default::default()
    };

    // This will fail without valid credentials, but tests the structure
    let result = utilities_api::balances_get(&config, None, None, None).await;
    // We expect an error without valid credentials
    assert!(result.is_err());
}
