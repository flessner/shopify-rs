use leptos::prelude::*;
use shopify_storefront::types::*;

#[server]
pub async fn get_product(handle: String) -> Result<Option<Product>, ServerFnError> {
    use shopify_storefront::{
        Client,
        queries::{ProductQuery, ProductQueryVariables},
    };

    let client = Client::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let operation = ProductQuery::build(ProductQueryVariables { handle });

    let product: Option<Product> = client
        .run(operation)
        .await
        .map(Option::<Product>::from)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(product)
}

#[server]
pub async fn get_products(first: i32) -> Result<Vec<Product>, ServerFnError> {
    use shopify_storefront::{
        Client,
        queries::{ProductsQuery, ProductsQueryVariables},
    };

    let client = Client::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;

    let operation = ProductsQuery::build(ProductsQueryVariables { first });

    let connection: ProductConnection = client
        .run(operation)
        .await
        .map(ProductConnection::from)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(connection.nodes)
}
