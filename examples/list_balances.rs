//! List balances example

use noah_sdk::{AuthConfig, Config, Environment, NoahClient};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    println!("Base URL: {}", client.base_url());
    println!("Requesting balances...");

    // Get balances
    let balances = client.get_balances(None, None).await?;

    println!("Found {} balances:", balances.items.len());
    for balance in balances.items {
        println!(
            "  {} {}: Available={}, Total={}",
            balance.crypto_currency, balance.account_type, balance.available, balance.total
        );
    }

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Get balances
    let balances = client.get_balances_blocking(None, None)?;

    println!("Found {} balances:", balances.items.len());
    for balance in balances.items {
        println!(
            "  {} {}: Available={}, Total={}",
            balance.crypto_currency, balance.account_type, balance.available, balance.total
        );
    }

    Ok(())
}
