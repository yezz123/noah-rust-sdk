//! Integration tests for the client

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
    use noah_sdk::{AuthConfig, Config, Environment, NoahClient};

    #[tokio::test]
    #[ignore] // Requires actual API credentials
    async fn test_client_creation() {
        let config = Config::new(Environment::Sandbox);
        let auth = AuthConfig::with_api_key("test-api-key".to_string());
        let client = NoahClient::new(config, auth);
        assert!(client.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires actual API credentials
    async fn test_get_balances() {
        let config = Config::new(Environment::Sandbox);
        let auth = AuthConfig::with_api_key("test-api-key".to_string());
        let client = NoahClient::new(config, auth).unwrap();

        // This will fail without valid credentials, but tests the structure
        let result = client.get_balances(None, None).await;
        // We expect an error without valid credentials
        assert!(result.is_err());
    }
}

