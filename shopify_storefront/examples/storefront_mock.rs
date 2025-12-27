use shopify_storefront::StorefrontClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading data from https://mock.shop/");
    let sf = StorefrontClient::new(
        "mock.shop".to_string(),
        "".to_string(), // Mock shop may not require a token
    );

    let shop = sf.fetch_shop().await?;
    println!("Shop name: {}", shop.name);
    println!("Description: {}", shop.description);

    let collections = sf.fetch_collections().await?;
    println!("Collections:");
    for collection in collections {
        println!("- {}", collection.title);
    }

    Ok(())
}
