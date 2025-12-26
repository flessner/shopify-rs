use leptos::prelude::*;
use shopify_rs::storefront::*;
use std::env;

#[cfg(not(target_family = "wasm"))]
fn client() -> StorefrontClient {
    let domain = env::var("SHOPIFY_DOMAIN").expect("SHOPIFY_DOMAIN must be set");
    let token = env::var("SHOPIFY_SF_TOKEN").expect("SHOPIFY_SF_TOKEN must be set");

    return StorefrontClient::new(domain, token);
}

pub fn use_shop() -> Resource<Option<StorefrontShop>> {
    Resource::new(
        || (),
        #[cfg(not(target_family = "wasm"))]
        |_| async move {
            let client = client();
            client.fetch_shop().await.ok()
        },
        #[cfg(target_family = "wasm")]
        |_| async move { None },
    )
}

pub fn use_collections() -> Resource<Option<Vec<StorefrontCollection>>> {
    Resource::new(
        || (),
        #[cfg(not(target_family = "wasm"))]
        |_| async move {
            let client = client();
            client.fetch_collections().await.ok()
        },
        #[cfg(target_family = "wasm")]
        |_| async move { None },
    )
}
