## Changelog

## 1.0.4

### Features

- Refactor Axum wrapper types to match our configuration.

## 1.0.3

### Features

- Add Axum wrapper types for deserialization and OpenAPI schema generation. PR [#4](https://github.com/yezz123/noah-rust-sdk/pull/4) by [@yezz123](https://github.com/yezz123).

## 1.0.2

- fix documenentation issues

## 1.0.1

### Features

- feat: Configure the sdk to have a clean documentation. PR [#3](https://github.com/yezz123/noah-rust-sdk/pull/3) by [@yezz123](https://github.com/yezz123).

## 1.0.0

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

- Initial SDK for Noah business. PR [#1](https://github.com/yezz123/noah-rust-sdk/pull/1) by [@yezz123](https://github.com/yezz123).

### Upgrades

- ⬆ Bump actions/checkout from 3 to 5 in the github-actions-packages group. PR [#2](https://github.com/yezz123/noah-rust-sdk/pull/2) by [@dependabot[bot]](https://github.com/apps/dependabot).
