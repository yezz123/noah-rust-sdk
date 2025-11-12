//! Sell transaction example

use noah_sdk::{
    models::common::*,
    models::transactions::{PrepareSellRequest, SellRequest},
    AuthConfig, Config, Environment, NoahClient,
};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Step 1: Prepare the sell transaction
    let prepare_request = PrepareSellRequest {
        channel_id: "channel-uuid-here".to_string(),
        payment_method_id: None,
        crypto_currency: CryptoCurrencyCode::Usdc,
        customer_id: Some("customer-123".to_string()),
        fiat_amount: "100.0".to_string(),
        form: None,
        delayed_sell: None,
    };

    let prepare_response = client.prepare_sell(&prepare_request).await?;
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
        crypto_currency: CryptoCurrencyCode::Usdc,
        fiat_amount: "100.0".to_string(),
        crypto_authorized_amount: prepare_response.crypto_authorized_amount,
        form_session_id: prepare_response.form_session_id,
        nonce: uuid::Uuid::new_v4().to_string(),
        external_id: Some("external-789".to_string()),
    };

    let sell_response = client.sell(&sell_request).await?;
    println!("Sell transaction created!");
    println!("  Transaction ID: {}", sell_response.transaction.id);
    println!("  Status: {:?}", sell_response.transaction.status);

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Step 1: Prepare the sell transaction
    let prepare_request = PrepareSellRequest {
        channel_id: "channel-uuid-here".to_string(),
        payment_method_id: None,
        crypto_currency: CryptoCurrencyCode::Usdc,
        customer_id: Some("customer-123".to_string()),
        fiat_amount: "100.0".to_string(),
        form: None,
        delayed_sell: None,
    };

    let prepare_response = client.prepare_sell_blocking(&prepare_request)?;
    println!("Prepared sell transaction:");
    println!("  Total Fee: {}", prepare_response.total_fee);
    println!(
        "  Crypto Amount Estimate: {}",
        prepare_response.crypto_amount_estimate
    );

    // Step 2: Execute the sell transaction
    let sell_request = SellRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        fiat_amount: "100.0".to_string(),
        crypto_authorized_amount: prepare_response.crypto_authorized_amount,
        form_session_id: prepare_response.form_session_id,
        nonce: uuid::Uuid::new_v4().to_string(),
        external_id: Some("external-789".to_string()),
    };

    let sell_response = client.sell_blocking(&sell_request)?;
    println!("Sell transaction created!");
    println!("  Transaction ID: {}", sell_response.transaction.id);

    Ok(())
}
