//! Basic client setup example

use noah_sdk::{AuthConfig, Config, Environment, NoahClient};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for sandbox environment
    let config = Config::new(Environment::Sandbox);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());

    // Create the client
    let client = NoahClient::new(config, auth)?;

    println!("Client created successfully!");
    println!("Base URL: {}", client.base_url());

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for sandbox environment
    let config = Config::new(Environment::Sandbox);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());

    // Create the client
    let client = NoahClient::new(config, auth)?;

    println!("Client created successfully!");
    println!("Base URL: {}", client.base_url());

    Ok(())
}
