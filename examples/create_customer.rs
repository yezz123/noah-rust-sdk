//! Create customer example

use noah_sdk::apis::configuration::{ApiKey, Configuration};
use noah_sdk::apis::{onboarding_api, utilities_api};
use noah_sdk::models::{CustomerInput, FullName, IndividualCustomerInput, StreetAddress};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::default();
    config.base_path = "https://api.sandbox.noah.com/v1".to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key-here".to_string(),
    });

    // Create an individual customer
    let customer_input = CustomerInput::Individual(Box::new(IndividualCustomerInput {
        r#type: noah_sdk::models::individual_customer_input::Type::Individual,
        full_name: Box::new(FullName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_name: None,
        }),
        date_of_birth: "1990-01-01".to_string(),
        email: Some("john.doe@example.com".to_string()),
        phone_number: Some("+1234567890".to_string()),
        identities: vec![],
        primary_residence: Box::new(StreetAddress {
            street: "123 Main St".to_string(),
            street2: None,
            city: "New York".to_string(),
            post_code: "10001".to_string(),
            state: "NY".to_string(),
            country: "US".to_string(),
        }),
    }));

    let customer_id = "customer-123".to_string();
    onboarding_api::customers_customer_id_put(&config, &customer_id, customer_input, None).await?;

    println!("Customer created successfully: {}", customer_id);

    // Retrieve the customer
    let customer = utilities_api::customers_customer_id_get(&config, &customer_id).await?;
    println!("Retrieved customer: {:?}", customer);

    Ok(())
}
