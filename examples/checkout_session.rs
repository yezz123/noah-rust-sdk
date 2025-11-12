//! Checkout session example

use noah_sdk::{
    api::checkout::{CryptoPayinRequest, FiatPayoutRequest},
    models::checkout::LineItem,
    models::common::*,
    AuthConfig, Config, Environment, NoahClient,
};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Example: Create a crypto payin session
    let crypto_payin = CryptoPayinRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        crypto_amount: "100.0".to_string(),
        return_url: "https://example.com/return".to_string(),
        customer_id: "customer-123".to_string(),
        external_id: Some("external-123".to_string()),
        customer: None,
        line_items: vec![LineItem {
            description: "Payment for services".to_string(),
            quantity: "1".to_string(),
            unit_amount: "100.0".to_string(),
            total_amount: "100.0".to_string(),
        }],
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    let session = client.create_crypto_payin_session(&crypto_payin).await?;
    println!("Checkout session created!");
    println!("Hosted URL: {}", session.hosted_url);
    println!(
        "Session ID: {}",
        session.checkout_session.checkout_session_id
    );

    // Example: Create a fiat payout session
    let fiat_payout = FiatPayoutRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        fiat_currency: "USD".to_string(),
        fiat_amount: "100.0".to_string(),
        crypto_authorized_amount: "100.0".to_string(),
        return_url: "https://example.com/return".to_string(),
        customer_id: "customer-123".to_string(),
        external_id: Some("external-456".to_string()),
        customer: None,
        line_items: vec![LineItem {
            description: "Payout to bank account".to_string(),
            quantity: "1".to_string(),
            unit_amount: "100.0".to_string(),
            total_amount: "100.0".to_string(),
        }],
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    let payout_session = client.create_fiat_payout_session(&fiat_payout).await?;
    println!("Payout session created!");
    println!("Hosted URL: {}", payout_session.hosted_url);

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Example: Create a crypto payin session
    let crypto_payin = CryptoPayinRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        crypto_amount: "100.0".to_string(),
        return_url: "https://example.com/return".to_string(),
        customer_id: "customer-123".to_string(),
        external_id: Some("external-123".to_string()),
        customer: None,
        line_items: vec![LineItem {
            description: "Payment for services".to_string(),
            quantity: "1".to_string(),
            unit_amount: "100.0".to_string(),
            total_amount: "100.0".to_string(),
        }],
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    let session = client.create_crypto_payin_session_blocking(&crypto_payin)?;
    println!("Checkout session created!");
    println!("Hosted URL: {}", session.hosted_url);

    Ok(())
}
