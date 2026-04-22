/// Leptos integration for the Shopify Storefront API.
///
/// Uses Leptos server functions so that all Shopify API calls execute on the
/// server only. Domain and token are resolved from environment variables
/// (`SHOPIFY_DOMAIN`, `SHOPIFY_SF_TOKEN`) so no credentials flow through
/// server function arguments.
///
/// Enable the `ssr` feature in your server binary and `leptos/hydrate` in
/// your WASM bundle.
mod functions;

pub use functions::*;
pub use shopify_storefront::types;
