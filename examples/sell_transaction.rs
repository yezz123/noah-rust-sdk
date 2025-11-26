//! Sell transaction example

use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::payout_api;
use noah_sdk::models::{PrepareSellRequest, SellRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    // Step 1: Prepare the sell transaction
    let prepare_request = PrepareSellRequest {
        channel_id: uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
        payment_method_id: None,
        crypto_currency: "USDC".to_string(),
        customer_id: Some("customer-123".to_string()),
        fiat_amount: "100.0".to_string(),
        form: None,
        delayed_sell: None,
    };

    let prepare_response =
        payout_api::transactions_sell_prepare_post(&config, prepare_request, None).await?;
    println!("Prepared sell transaction:");
    println!("  Total Fee: {}", prepare_response.total_fee);
    println!(
        "  Crypto Amount Estimate: {}",
        prepare_response.crypto_amount_estimate
    );
    println!(
        "  Crypto Authorized Amount: {}",
        prepare_response.crypto_authorized_amount
    );
    println!("  Form Session ID: {}", prepare_response.form_session_id);

    // Step 2: Execute the sell transaction
    let sell_request = SellRequest {
        crypto_currency: "USDC".to_string(),
        fiat_amount: "100.0".to_string(),
        crypto_authorized_amount: prepare_response.crypto_authorized_amount,
        form_session_id: prepare_response.form_session_id,
        nonce: uuid::Uuid::new_v4().to_string(),
        external_id: Some("external-789".to_string()),
    };

    let sell_response = payout_api::transactions_sell_post(&config, sell_request, None).await?;
    println!("Sell transaction created!");
    println!("  Transaction ID: {}", sell_response.transaction.id);
    println!("  Status: {:?}", sell_response.transaction.status);

    Ok(())
}
