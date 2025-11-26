//! Basic client setup example

use noah_sdk::apis::configuration::{ApiKey, Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for sandbox environment
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    println!("Client created successfully!");
    println!("Base URL: {}", config.base_path);

    Ok(())
}
