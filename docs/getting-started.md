# Getting Started with Noah SDK

This guide will help you get started with the Noah SDK for Rust.

## Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
noah-sdk = { version = "0.1.0", features = ["async"] }
tokio = { version = "1.0", features = ["full"] }
```

## Basic Setup

### 1. Create a Client

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};

let config = Config::new(Environment::Sandbox);
let auth = AuthConfig::with_api_key("your-api-key".to_string());
let client = NoahClient::new(config, auth)?;
```

### 2. Make Your First Request

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    let client = NoahClient::new(config, auth)?;

    // Get balances
    let balances = client.get_balances(None, None).await?;

    for balance in balances.items {
        println!("{}: {} available", balance.crypto_currency, balance.available);
    }

    Ok(())
}
```

## Environment Selection

Choose between sandbox and production:

```rust
// Sandbox (for testing)
let config = Config::new(Environment::Sandbox);

// Production
let config = Config::new(Environment::Production);
```

## Authentication

### Using API Key

```rust
let auth = AuthConfig::with_api_key("your-api-key".to_string());
```

### Using JWT Signing

```rust
let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
```

The SDK will automatically sign requests with JWT when using a secret key.

## Next Steps

- Read the [Authentication Guide](authentication.md) for detailed auth setup
- Check out [Examples](examples.md) for common use cases
- Review the [Error Handling Guide](error-handling.md) for robust error management
