//! Unit tests for models

use noah_sdk::models::BalanceResponse;

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
    assert_eq!(balance.crypto_currency, "USDC");
    assert_eq!(balance.account_type, "Current");
}
