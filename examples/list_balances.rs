//! List balances example

use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::utilities_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    println!("Base URL: {}", config.base_path);
    println!("Requesting balances...");

    // Get balances
    let balances = utilities_api::balances_get(&config, None, None, None).await?;

    println!("Found {} balances:", balances.items.len());
    for balance in balances.items {
        println!(
            "  {} {}: Available={}, Total={}",
            balance.crypto_currency, balance.account_type, balance.available, balance.total
        );
    }

    Ok(())
}
