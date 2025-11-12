# Noah SDK

![Noah SDK](./docs/images/logo.png)

A modern, type-safe Rust SDK for the [Noah Business API](https://noah.com).

## Features

- ✅ **Async and Sync Support**: Use async/await or blocking operations via feature flags
- ✅ **Type Safety**: Strongly typed models generated from OpenAPI schema
- ✅ **Dual Authentication**: Support for both JWT signing (`Api-Signature`) and API key (`X-Api-Key`) authentication
- ✅ **Comprehensive Error Handling**: Detailed error types with context
- ✅ **Production Ready**: Well-tested and documented

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noah-sdk = { version = "0.1.0", features = ["async"] }
```

For blocking/sync operations:

```toml
[dependencies]
noah-sdk = { version = "0.1.0", features = ["sync"] }
```

## Quick Start

### Async Example

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration for sandbox environment
    let config = Config::new(Environment::Sandbox);

    // Create authentication config with API key
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());

    // Create the client
    let client = NoahClient::new(config, auth)?;

    // Get balances
    let balances = client.get_balances(None, None).await?;
    println!("Found {} balances", balances.items.len());

    Ok(())
}
```

### Sync Example

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    let balances = client.get_balances_blocking(None, None)?;
    println!("Found {} balances", balances.items.len());

    Ok(())
}
```

## Authentication

The SDK supports two authentication methods:

### API Key Authentication

```rust
let auth = AuthConfig::with_api_key("your-api-key".to_string());
```

### JWT Signature Authentication

```rust
let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
```

### Both Methods

```rust
let auth = AuthConfig::with_both(
    "your-api-key".to_string(),
    "your-secret-key".to_string()
);
```

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
cargo run --example basic_client --features async
```

## API Coverage

The SDK provides typed methods for all major Noah API endpoints:

- **Balances**: Get account balances
- **Channels**: List and get channel information
- **Customers**: Create, update, and retrieve customers
- **Transactions**: Prepare and execute sell transactions
- **Checkout**: Create payin and payout checkout sessions
- **Onboarding**: Create onboarding sessions and prefill customer data
- **Payment Methods**: List customer payment methods
- **Workflows**: Create automated workflows

## Error Handling

The SDK uses a comprehensive error type system:

```rust
use noah_sdk::error::NoahError;

match client.get_balances(None, None).await {
    Ok(balances) => println!("Success: {:?}", balances),
    Err(NoahError::ApiError(err)) => println!("API error: {}", err),
    Err(NoahError::HttpError(err)) => println!("HTTP error: {}", err),
    Err(NoahError::AuthError(msg)) => println!("Auth error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Configuration

### Environment Selection

```rust
// Sandbox (default)
let config = Config::new(Environment::Sandbox);

// Production
let config = Config::new(Environment::Production);

// Custom URL
let config = Config::new(Environment::Custom(
    url::Url::parse("https://api.custom.com/v1")?
));
```

### Custom Configuration

```rust
let config = Config::new(Environment::Sandbox)
    .with_timeout(60)  // 60 second timeout
    .with_user_agent("my-app/1.0".to_string())
    .with_logging(true);
```

## Features

- `default`: Enables async support with rustls-tls
- `async`: Async/await support (requires tokio)
- `sync`: Blocking/synchronous operations
- `rustls-tls`: Use rustls for TLS (default)
- `native-tls`: Use native TLS implementation
- `json`: JSON serialization support (enabled by default)

## Documentation

- [Getting Started](docs/getting-started.md)
- [Authentication](docs/authentication.md)
- [Error Handling](docs/error-handling.md)
- [Examples](docs/examples.md)

## License

Licensed under the MIT license ([LICENSE](LICENSE)).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
