# Authentication

The Noah SDK supports two authentication methods:

1. **API Key** (`X-Api-Key` header)
2. **JWT Signature** (`Api-Signature` header)

## API Key Authentication

The simplest authentication method is using an API key:

```rust
use noah_sdk::AuthConfig;

let auth = AuthConfig::with_api_key("your-api-key".to_string());
```

The SDK will automatically include the `X-Api-Key` header in all requests.

## JWT Signature Authentication

For enhanced security, you can use JWT signature authentication:

```rust
use noah_sdk::AuthConfig;

let auth = AuthConfig::with_secret_key("your-secret-key".to_string());
```

The SDK will automatically:

1. Generate a JWT token for each request
2. Sign it with HMAC-SHA256 using your secret key
3. Include it in the `Api-Signature` header

The JWT includes:

- HTTP method
- Request path
- Request body hash (for POST/PUT requests)
- Timestamp and expiration

## Using Both Methods

You can use both authentication methods simultaneously:

```rust
let auth = AuthConfig::with_both(
    "your-api-key".to_string(),
    "your-secret-key".to_string()
);
```

This is useful when you want to use API key for simplicity but also have JWT signing for additional security.

## Getting Your Credentials

1. **API Key**: Obtain from your Noah dashboard
2. **Secret Key**: Used for JWT signing, also from your Noah dashboard

Keep your credentials secure and never commit them to version control. Use environment variables or a secrets manager in production.

## Example

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment
    let api_key = env::var("NOAH_API_KEY")?;
    let secret_key = env::var("NOAH_SECRET_KEY").ok();

    let config = Config::new(Environment::Sandbox);

    let auth = if let Some(secret) = secret_key {
        AuthConfig::with_both(api_key, secret)
    } else {
        AuthConfig::with_api_key(api_key)
    };

    let client = NoahClient::new(config, auth)?;

    // Make authenticated requests
    let balances = client.get_balances(None, None).await?;

    Ok(())
}
```
