//! Webhook signature verification example

use noah_sdk::models::checkout::CheckoutSession;
use serde_json::Value;

/// Example webhook handler that verifies webhook signatures
///
/// Note: This is a simplified example. In production, you should:
/// 1. Verify the Webhook-Signature header
/// 2. Validate the request body
/// 3. Handle different event types appropriately
fn handle_webhook(body: &str, signature: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // In production, verify the signature here
    if let Some(sig) = signature {
        println!("Received signature: {}", sig);
        // TODO: Implement signature verification
    }

    // Parse the webhook payload
    let payload: Value = serde_json::from_str(body)?;

    // Extract event information
    let event_type = payload["EventType"].as_str().unwrap_or("Unknown");
    let event_version = payload["EventVersion"].as_u64().unwrap_or(0);
    let occurred = payload["Occurred"].as_str().unwrap_or("Unknown");

    println!("Webhook received:");
    println!("  Event Type: {}", event_type);
    println!("  Event Version: {}", event_version);
    println!("  Occurred: {}", occurred);

    // Handle different event types
    match event_type {
        "Transaction" => {
            println!("  Processing transaction event...");
            // Handle transaction event
        }
        "CheckoutSession" => {
            println!("  Processing checkout session event...");
            if let Ok(session) = serde_json::from_value::<CheckoutSession>(payload["Data"].clone())
            {
                println!("    Session ID: {}", session.checkout_session_id);
                println!("    Status: {}", session.status);
            }
        }
        "Customer" => {
            println!("  Processing customer event...");
            // Handle customer event
        }
        _ => {
            println!("  Unknown event type: {}", event_type);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example webhook payload (in production, this would come from HTTP request)
    let example_payload = r#"
    {
        "EventType": "CheckoutSession",
        "EventVersion": 1,
        "Occurred": "2024-01-01T00:00:00Z",
        "Data": {
            "CheckoutSessionID": "session-123",
            "Status": "Settled",
            "CustomerID": "customer-123",
            "Type": "PayinCrypto",
            "SourceCurrency": "USDC",
            "DestinationCurrency": "USD",
            "SourceAmount": "100.0",
            "DestinationAmount": "100.0",
            "ReturnURL": "https://example.com/return",
            "LineItems": [],
            "Created": "2024-01-01T00:00:00Z"
        },
        "UserID": "user-123"
    }
    "#;

    handle_webhook(example_payload, Some("example-signature"))?;

    Ok(())
}
