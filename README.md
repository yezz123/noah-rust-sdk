# Noah SDK

![Noah SDK](https://github.com/yezz123/noah-rust-sdk/blob/main/docs/images/logo.png?raw=true)

[![Documentation](https://docs.rs/noah-sdk/badge.svg)](https://docs.rs/noah-sdk)
[![Crates.io](https://img.shields.io/crates/v/noah-sdk.svg)](https://crates.io/crates/noah-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.60+-black.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/noah-sdk)
[![CI](https://github.com/yezz123/noah-rust-sdk/actions/workflows/ci.yaml/badge.svg)](https://github.com/yezz123/noah-rust-sdk/actions/workflows/ci.yaml)
[![dependency status](https://deps.rs/repo/github/yezz123/noah-rust-sdk/status.svg)](https://deps.rs/repo/github/yezz123/noah-rust-sdk)

---

A modern, type-safe Rust SDK for the [Noah Business API](https://noah.com).

## Features

- ✅ **Type Safety**: Strongly typed models generated from OpenAPI schema
- ✅ **Async Support**: Built on async/await with `reqwest`
- ✅ **Dual Authentication**: Support for both JWT signing (`Api-Signature`) and API key (`X-Api-Key`) authentication
- ✅ **Comprehensive Error Handling**: Detailed error types with context
- ✅ **Production Ready**: Well-tested and documented

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noah-sdk = { version = "1.0", features = ["rustls-tls"] }
```

For native TLS:

```toml
[dependencies]
noah-sdk = { version = "1.0", features = ["native-tls"] }
```

## Quick Start

### Basic Example

```rust
use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::utilities_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for sandbox environment
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    // Get balances
    let balances = utilities_api::balances_get(&config, None, None, None).await?;
    println!("Found {} balances", balances.items.len());

    Ok(())
}
```

## Authentication

The SDK supports two authentication methods:

### API Key Authentication

```rust
use noah_sdk::apis::configuration::{ApiKey, Configuration};

let mut config = Configuration::default();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "your-api-key".to_string(),
});
```

### JWT Signature Authentication

```rust
let mut config = Configuration::default();
config.bearer_access_token = Some("your-jwt-token".to_string());
// Or pass api_signature parameter to API calls
```

### Both Methods

You can use both by setting both `api_key` and passing `api_signature` to individual API calls.

## Examples

See the [examples](examples/) directory for more detailed examples:

- [Basic Client Setup](examples/basic_client.rs)
- [List Balances](examples/list_balances.rs)
- [Create Customer](examples/create_customer.rs)
- [Checkout Session](examples/checkout_session.rs)
- [Sell Transaction](examples/sell_transaction.rs)
- [Webhook Handler](examples/webhook_handler.rs)

Run an example:

```bash
cargo run --example basic_client
```

## API Coverage

The SDK provides typed methods for all major Noah API endpoints:

- **Balances**: Get account balances (`utilities_api::balances_get`)
- **Channels**: List and get channel information (`utilities_api::channels_sell_get`, `utilities_api::channels_channel_id_get`)
- **Customers**: Create, update, and retrieve customers (`onboarding_api::customers_customer_id_put`, `utilities_api::customers_customer_id_get`)
- **Transactions**: Prepare and execute sell transactions (`payout_api::transactions_sell_prepare_post`, `payout_api::transactions_sell_post`)
- **Checkout**: Create payin and payout checkout sessions (`payin_api::checkout_payin_crypto_post`, `payout_api::checkout_payout_fiat_post`)
- **Onboarding**: Create onboarding sessions and prefill customer data (`onboarding_api::onboarding_customer_id_post`)
- **Payment Methods**: List customer payment methods (`utilities_api::payment_methods_get`)
- **Workflows**: Create automated workflows (`payin_api::workflows_bank_deposit_to_onchain_address_post`)

## Error Handling

The SDK uses comprehensive error types for each API endpoint:

```rust
use noah_sdk::apis::utilities_api;

match utilities_api::balances_get(&config, None, None, None).await {
    Ok(balances) => println!("Success: {:?}", balances),
    Err(noah_sdk::apis::Error::ResponseError(err)) => {
        println!("API error: status={}, content={}", err.status, err.content);
    }
    Err(noah_sdk::apis::Error::Reqwest(err)) => println!("HTTP error: {}", err),
    Err(noah_sdk::apis::Error::Serde(err)) => println!("Serialization error: {}", err),
    Err(e) => println!("Other error: {}", e),
}
```

## Configuration

### Environment Selection

```rust
use noah_sdk::apis::configuration::Configuration;

// Sandbox (default)
let mut config = Configuration::default();
config.base_path = "https://api.sandbox.noah.com/v1".to_string();

// Production
let mut config = Configuration::default();
config.base_path = "https://api.noah.com/v1".to_string();

// Custom URL
let mut config = Configuration::default();
config.base_path = "https://api.custom.com/v1".to_string();
```

### Custom Configuration

```rust
let mut config = Configuration::default();
config.base_path = "https://api.sandbox.noah.com/v1".to_string();
config.user_agent = Some("my-app/1.0".to_string());
config.api_key = Some(ApiKey {
    prefix: None,
    key: "your-api-key".to_string(),
});
```

## Features

- `default`: Enables native-tls
- `rustls-tls`: Use rustls for TLS
- `native-tls`: Use native TLS implementation (default)

## License

Licensed under the MIT license ([LICENSE](LICENSE)).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
