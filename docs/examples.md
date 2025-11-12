# Examples

This document provides examples for common use cases with the Noah SDK.

## List Balances

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let balances = client.get_balances(None, None).await?;

    for balance in balances.items {
        println!(
            "{} {}: {} available, {} total",
            balance.crypto_currency,
            balance.account_type,
            balance.available,
            balance.total
        );
    }

    Ok(())
}
```

## Create a Customer

```rust
use noah_sdk::{
    models::customers::{CustomerInput, IndividualCustomerInput, FullName, StreetAddress},
    NoahClient, Config, Environment, AuthConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let customer = CustomerInput::Individual(IndividualCustomerInput {
        customer_type: "Individual".to_string(),
        full_name: FullName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_name: None,
        },
        date_of_birth: "1990-01-01".to_string(),
        email: Some("john@example.com".to_string()),
        phone_number: Some("+1234567890".to_string()),
        identities: vec![],
        primary_residence: StreetAddress {
            street: "123 Main St".to_string(),
            street2: None,
            city: "New York".to_string(),
            post_code: "10001".to_string(),
            state: "NY".to_string(),
            country: "US".to_string(),
        },
    });

    client.create_or_update_customer("customer-123", &customer).await?;

    Ok(())
}
```

## Create a Checkout Session

```rust
use noah_sdk::{
    api::checkout::CryptoPayinRequest,
    models::checkout::LineItem,
    models::common::*,
    NoahClient, Config, Environment, AuthConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let request = CryptoPayinRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        crypto_amount: "100.0".to_string(),
        return_url: "https://example.com/return".to_string(),
        customer_id: "customer-123".to_string(),
        external_id: Some("external-123".to_string()),
        customer: None,
        line_items: vec![LineItem {
            description: "Payment for services".to_string(),
            quantity: "1".to_string(),
            unit_amount: "100.0".to_string(),
            total_amount: "100.0".to_string(),
        }],
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    let session = client.create_crypto_payin_session(&request).await?;
    println!("Checkout URL: {}", session.hosted_url);

    Ok(())
}
```

## Execute a Sell Transaction

```rust
use noah_sdk::{
    models::transactions::{PrepareSellRequest, SellRequest},
    models::common::*,
    NoahClient, Config, Environment, AuthConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    // Step 1: Prepare
    let prepare = PrepareSellRequest {
        channel_id: "channel-uuid".to_string(),
        payment_method_id: None,
        crypto_currency: CryptoCurrencyCode::Usdc,
        customer_id: Some("customer-123".to_string()),
        fiat_amount: "100.0".to_string(),
        form: None,
        delayed_sell: None,
    };

    let prepared = client.prepare_sell(&prepare).await?;

    // Step 2: Execute
    let sell = SellRequest {
        crypto_currency: CryptoCurrencyCode::Usdc,
        fiat_amount: "100.0".to_string(),
        crypto_authorized_amount: prepared.crypto_authorized_amount,
        form_session_id: prepared.form_session_id,
        nonce: uuid::Uuid::new_v4().to_string(),
        external_id: Some("external-123".to_string()),
    };

    let result = client.sell(&sell).await?;
    println!("Transaction ID: {}", result.transaction.id);

    Ok(())
}
```

## Pagination

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let mut page_token: Option<String> = None;
    let mut all_items = Vec::new();

    loop {
        let response = client.get_balances(Some(20), page_token.as_deref()).await?;

        all_items.extend(response.items);

        page_token = response.page_token;
        if page_token.is_none() {
            break;
        }
    }

    println!("Total items: {}", all_items.len());

    Ok(())
}
```

## Error Handling

```rust
use noah_sdk::{NoahClient, Config, Environment, AuthConfig, error::NoahError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    match client.get_customer("invalid-id").await {
        Ok(customer) => println!("Customer: {:?}", customer),
        Err(NoahError::ApiError(err)) => {
            println!("API error: {:?}", err.error_type);
            if let Some(detail) = err.detail {
                println!("Detail: {}", detail);
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

fn create_client() -> Result<NoahClient, Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key".to_string());
    Ok(NoahClient::new(config, auth)?)
}
```
