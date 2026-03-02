//! Demonstrates querying the Shopify Customer Account API with a typed cynic query.
//!
//! Uses Shopify's `demostore.mock.shop` as the shop domain. Obtain an OAuth
//! access token by logging in at https://demostore.mock.shop and supplying it
//! through an environment variable:
//!
//! ```sh
//! SHOPIFY_CA_TOKEN=your_access_token cargo run --example ca_mock
//! ```

use cynic::QueryBuilder;
use shopify_customer_account::Client;

#[cynic::schema("ca")]
mod schema {}

#[derive(cynic::Scalar, Debug)]
#[cynic(graphql_type = "URL")]
struct Url(String);

#[derive(cynic::QueryFragment, Debug)]
struct CustomerEmailAddress {
    email_address: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
struct Customer {
    id: cynic::Id,
    display_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email_address: Option<CustomerEmailAddress>,
    image_url: Url,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
struct CustomerQuery {
    customer: Customer,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("SHOPIFY_CA_TOKEN")
        .expect("SHOPIFY_CA_TOKEN environment variable not set");

    let client = Client::new("demostore.mock.shop");
    let data = client.run(CustomerQuery::build(()), &access_token).await?;

    let c = data.customer;
    println!("ID:           {}", c.id.inner());
    println!("Display name: {}", c.display_name);
    println!(
        "Name:         {} {}",
        c.first_name.as_deref().unwrap_or(""),
        c.last_name.as_deref().unwrap_or(""),
    );
    println!(
        "Email:        {}",
        c.email_address
            .and_then(|e| e.email_address)
            .as_deref()
            .unwrap_or("(none)"),
    );
    println!("Avatar:       {}", c.image_url.0);

    Ok(())
}
