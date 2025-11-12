//! Unit tests for models

#[cfg(test)]
mod tests {
    use noah_sdk::models::common::*;
    use noah_sdk::models::balances::BalanceResponse;
    use serde_json;

    #[test]
    fn test_balance_response_deserialization() {
        let json = r#"
        {
            "AccountType": "Current",
            "CryptoCurrency": "USDC",
            "Available": "1000.0",
            "Total": "1000.0"
        }
        "#;

        let balance: BalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(balance.available, "1000.0");
        assert_eq!(balance.total, "1000.0");
    }

    #[test]
    fn test_crypto_currency_serialization() {
        let currency = CryptoCurrencyCode::Usdc;
        let json = serde_json::to_string(&currency).unwrap();
        assert!(json.contains("USDC"));
    }

    #[test]
    fn test_network_enum() {
        let network = Network::Ethereum;
        let json = serde_json::to_string(&network).unwrap();
        assert!(json.contains("Ethereum"));
    }
}

