# Error Handling

The Noah SDK provides comprehensive error handling with detailed error types.

## Error Types

The main error type is `NoahError`, which includes several variants:

```rust
use noah_sdk::error::NoahError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(NoahError::HttpError(e)) => {
        // Network or HTTP client errors
        println!("HTTP error: {}", e);
    }
    Err(NoahError::ApiError(e)) => {
        // API error responses from Noah
        println!("API error: {} - {}", e.error_type, e.detail.unwrap_or_default());
    }
    Err(NoahError::AuthError(msg)) => {
        // Authentication failures
        println!("Auth error: {}", msg);
    }
    Err(NoahError::ValidationError(msg)) => {
        // Request validation errors
        println!("Validation error: {}", msg);
    }
    Err(NoahError::DeserializationError(e)) => {
        // JSON parsing errors
        println!("Deserialization error: {}", e);
    }
    Err(NoahError::JwtError(msg)) => {
        // JWT signing errors
        println!("JWT error: {}", msg);
    }
    Err(e) => {
        // Other errors
        println!("Error: {}", e);
    }
}
```

## API Error Responses

When the API returns an error, it's parsed into `ApiErrorResponse`:

```rust
use noah_sdk::error::{NoahError, ApiErrorResponse};

match client.get_balances(None, None).await {
    Err(NoahError::ApiError(api_err)) => {
        println!("Error type: {:?}", api_err.error_type);
        println!("Detail: {:?}", api_err.detail);
        println!("Instance: {:?}", api_err.instance);

        // Check for request extension (field-level errors)
        if let Some(ref req_ext) = api_err.request_extension {
            for item in &req_ext.body {
                println!("Field '{}': {}", item.field, item.description);
            }
        }
    }
    _ => {}
}
```

## Best Practices

### 1. Use `?` for Error Propagation

```rust
fn my_function() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;
    let balances = client.get_balances(None, None).await?;
    Ok(())
}
```

### 2. Handle Specific Errors

```rust
match client.get_customer(&customer_id).await {
    Ok(customer) => println!("Customer: {:?}", customer),
    Err(NoahError::ApiError(err)) if err.error_type == ErrorType::ResourceNotFound => {
        println!("Customer not found");
    }
    Err(e) => return Err(e.into()),
}
```

### 3. Log Errors with Context

```rust
use tracing::error;

match client.sell(&sell_request).await {
    Ok(response) => Ok(response),
    Err(e) => {
        error!("Failed to execute sell transaction: {}", e);
        Err(e)
    }
}
```

## Error Conversion

All errors implement `std::error::Error` and can be converted to `anyhow::Error`:

```rust
use anyhow::Result;

fn my_function() -> Result<()> {
    let client = create_client()?;
    let balances = client.get_balances(None, None).await?;
    Ok(())
}
```

## Common Error Scenarios

### Network Errors

```rust
match client.get_balances(None, None).await {
    Err(NoahError::HttpError(e)) if e.is_timeout() => {
        println!("Request timed out");
    }
    Err(NoahError::HttpError(e)) if e.is_connect() => {
        println!("Connection failed");
    }
    _ => {}
}
```

### Authentication Errors

```rust
match client.get_balances(None, None).await {
    Err(NoahError::ApiError(err)) if err.error_type == ErrorType::Unauthorized => {
        println!("Invalid credentials");
    }
    Err(NoahError::ApiError(err)) if err.error_type == ErrorType::Forbidden => {
        println!("Insufficient permissions");
    }
    _ => {}
}
```
