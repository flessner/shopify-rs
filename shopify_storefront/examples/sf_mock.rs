//! Demonstrates querying Shopify's mock.shop with a typed cynic query.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example storefront_mock
//! ```

use cynic::QueryBuilder;
use shopify_storefront::Client;

#[cynic::schema("storefront")]
mod schema {}

#[derive(cynic::Scalar, Debug)]
#[cynic(graphql_type = "Decimal")]
struct Decimal(String);

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(non_exhaustive)]
enum CurrencyCode {
    Usd,
    Cad,
    Eur,
    Gbp,
    Aud,
    #[cynic(fallback)]
    Other,
}

#[derive(cynic::QueryFragment, Debug)]
struct MoneyV2 {
    amount: Decimal,
    currency_code: CurrencyCode,
}

#[derive(cynic::QueryFragment, Debug)]
struct ProductPriceRange {
    min_variant_price: MoneyV2,
}

#[derive(cynic::QueryFragment, Debug)]
struct Product {
    title: String,
    handle: String,
    price_range: ProductPriceRange,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
struct ProductsQuery {
    #[arguments(first: 5)]
    products: ProductConnection,
}

#[derive(cynic::QueryFragment, Debug)]
struct ProductConnection {
    nodes: Vec<Product>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("mock.shop");

    let data = client.run(ProductsQuery::build(())).await?;

    for p in data.products.nodes {
        let price = p.price_range.min_variant_price;
        println!(
            "{} ({})\n  from {} {:?}\n",
            p.title, p.handle, price.amount.0, price.currency_code,
        );
    }

    Ok(())
}
