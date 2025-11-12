//! Create customer example

use noah_sdk::{
    models::customers::{CustomerInput, FullName, IndividualCustomerInput, StreetAddress},
    AuthConfig, Config, Environment, NoahClient,
};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Create an individual customer
    let customer_input = CustomerInput::Individual(IndividualCustomerInput {
        customer_type: "Individual".to_string(),
        full_name: FullName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_name: None,
        },
        date_of_birth: "1990-01-01".to_string(),
        email: Some("john.doe@example.com".to_string()),
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

    let customer_id = "customer-123".to_string();
    client
        .create_or_update_customer(&customer_id, &customer_input)
        .await?;

    println!("Customer created successfully: {}", customer_id);

    // Retrieve the customer
    let customer = client.get_customer(&customer_id).await?;
    println!("Retrieved customer: {:?}", customer);

    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Environment::Sandbox);
    let auth = AuthConfig::with_api_key("your-api-key-here".to_string());
    let client = NoahClient::new(config, auth)?;

    // Create an individual customer
    let customer_input = CustomerInput::Individual(IndividualCustomerInput {
        customer_type: "Individual".to_string(),
        full_name: FullName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_name: None,
        },
        date_of_birth: "1990-01-01".to_string(),
        email: Some("john.doe@example.com".to_string()),
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

    let customer_id = "customer-123".to_string();
    client.create_or_update_customer_blocking(&customer_id, &customer_input)?;

    println!("Customer created successfully: {}", customer_id);

    // Retrieve the customer
    let customer = client.get_customer_blocking(&customer_id)?;
    println!("Retrieved customer: {:?}", customer);

    Ok(())
}
