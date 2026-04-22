//! Demonstrates querying the Shopify Customer Account API with a typed cynic query.
//!
//! Uses Shopify's `demostore.mock.shop` as the shop domain. The OpenID and
//! Customer Account configurations are fetched without any token. To also
//! query the authenticated customer endpoint, set the access token:
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
    let client = Client::new("demostore.mock.shop");

    // --- Unauthenticated: OpenID configuration ---
    let oidc = client.fetch_openid_config().await?;
    println!("=== OpenID Configuration ===");
    println!("Issuer:                 {}", oidc.issuer);
    println!("Authorization endpoint: {}", oidc.authorization_endpoint);
    println!("Token endpoint:         {}", oidc.token_endpoint);
    println!("End-session endpoint:   {}", oidc.end_session_endpoint);
    println!("JWKS URI:               {}", oidc.jwks_uri);

    // --- Unauthenticated: Customer Account API configuration ---
    let ca_config = client.fetch_customer_account_config().await?;
    println!("\n=== Customer Account API Configuration ===");
    println!("GraphQL API: {}", ca_config.graphql_api);
    println!("MCP API:     {}", ca_config.mcp_api);

    // --- Authenticated: customer profile (requires CA token) ---
    let access_token = match std::env::var("SHOPIFY_CA_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            println!("\nSHOPIFY_CA_TOKEN not set — skipping authenticated customer query.");
            return Ok(());
        }
    };

    let data = client.run(CustomerQuery::build(()), &access_token).await?;

    let c = data.customer;
    println!("\n=== Customer ===");
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
