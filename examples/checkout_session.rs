//! Checkout session example

use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::{payin_api, payout_api};
use noah_sdk::models::{CheckoutPayinCryptoPostRequest, CheckoutPayoutFiatPostRequest, LineItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    // Example: Create a crypto payin session
    let crypto_payin = CheckoutPayinCryptoPostRequest {
        crypto_currency: "USDC".to_string(),
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

    let session = payin_api::checkout_payin_crypto_post(&config, crypto_payin, None).await?;
    println!("Checkout session created!");
    println!("Hosted URL: {}", session.hosted_url);
    println!(
        "Session ID: {}",
        session.checkout_session.checkout_session_id
    );

    // Example: Create a fiat payout session
    let fiat_payout = CheckoutPayoutFiatPostRequest {
        crypto_currency: "USDC".to_string(),
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

    let payout_session = payout_api::checkout_payout_fiat_post(&config, fiat_payout, None).await?;
    println!("Payout session created!");
    println!("Hosted URL: {}", payout_session.hosted_url);

    Ok(())
}
