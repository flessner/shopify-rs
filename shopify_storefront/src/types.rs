/// Public data types for the Shopify Storefront API.
///
/// These are plain serializable structs shared between the server and any
/// consumer (including WASM). They are deliberately free of cynic or reqwest
/// dependencies.
use serde::{Deserialize, Serialize};

// ── Shared types ──────────────────────────────────────────────────────────────

/// A monetary value with amount and currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyV2 {
    pub amount: String,
    pub currency_code: String,
}

/// The price range of a product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPriceRange {
    pub min_variant_price: MoneyV2,
    pub max_variant_price: MoneyV2,
}

/// An image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub alt_text: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// A Shopify product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub description: String,
    pub vendor: String,
    pub product_type: String,
    pub tags: Vec<String>,
    pub featured_image: Option<Image>,
    pub price_range: ProductPriceRange,
}

/// A list of products.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductConnection {
    pub nodes: Vec<Product>,
}

/// A Shopify collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub description: String,
    pub image: Option<Image>,
}

/// A list of collections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConnection {
    pub nodes: Vec<Collection>,
}

/// Basic shop information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
