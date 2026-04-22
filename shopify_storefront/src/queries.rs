/// Cynic query fragments and conversions for the Shopify Storefront API.
///
/// Each public query type implements `cynic::QueryBuilder` via the derive macro
/// and can be built with `MyQuery::build(vars)` to produce a
/// `cynic::Operation` ready to pass to [`crate::Client::run`].
use crate::types::{
    Collection, CollectionConnection, Image, MoneyV2, Product, ProductConnection,
    ProductPriceRange, Shop,
};

// ── Schema registration ───────────────────────────────────────────────────────

#[cynic::schema("storefront")]
mod schema {}

// ── Scalars ───────────────────────────────────────────────────────────────────

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "Decimal")]
struct Decimal(String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "URL")]
struct Url(String);

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

// ── Shared fragments ──────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "MoneyV2")]
struct MoneyV2Fragment {
    amount: Decimal,
    currency_code: CurrencyCode,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "ProductPriceRange")]
struct ProductPriceRangeFragment {
    min_variant_price: MoneyV2Fragment,
    max_variant_price: MoneyV2Fragment,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Image")]
struct ImageFragment {
    url: Url,
    alt_text: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Product")]
struct ProductFragment {
    id: cynic::Id,
    title: String,
    handle: String,
    description: String,
    vendor: String,
    product_type: String,
    tags: Vec<String>,
    featured_image: Option<ImageFragment>,
    price_range: ProductPriceRangeFragment,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "ProductConnection")]
struct ProductConnectionFragment {
    nodes: Vec<ProductFragment>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Collection")]
struct CollectionFragment {
    id: cynic::Id,
    title: String,
    handle: String,
    description: String,
    image: Option<ImageFragment>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "CollectionConnection")]
struct CollectionConnectionFragment {
    nodes: Vec<CollectionFragment>,
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<MoneyV2Fragment> for MoneyV2 {
    fn from(f: MoneyV2Fragment) -> Self {
        Self {
            amount: f.amount.0,
            currency_code: format!("{:?}", f.currency_code),
        }
    }
}

impl From<ProductPriceRangeFragment> for ProductPriceRange {
    fn from(f: ProductPriceRangeFragment) -> Self {
        Self {
            min_variant_price: f.min_variant_price.into(),
            max_variant_price: f.max_variant_price.into(),
        }
    }
}

impl From<ImageFragment> for Image {
    fn from(f: ImageFragment) -> Self {
        Self {
            url: f.url.0,
            alt_text: f.alt_text,
            width: f.width,
            height: f.height,
        }
    }
}

impl From<ProductFragment> for Product {
    fn from(f: ProductFragment) -> Self {
        Self {
            id: f.id.into_inner(),
            title: f.title,
            handle: f.handle,
            description: f.description,
            vendor: f.vendor,
            product_type: f.product_type,
            tags: f.tags,
            featured_image: f.featured_image.map(Image::from),
            price_range: f.price_range.into(),
        }
    }
}

impl From<CollectionFragment> for Collection {
    fn from(f: CollectionFragment) -> Self {
        Self {
            id: f.id.into_inner(),
            title: f.title,
            handle: f.handle,
            description: f.description,
            image: f.image.map(Image::from),
        }
    }
}

// ── Shop query ────────────────────────────────────────────────────────────────

/// Variables for the [`ShopQuery`].
#[derive(cynic::QueryVariables, Debug, Clone)]
pub struct ShopQueryVariables {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Shop")]
struct ShopFragment {
    id: cynic::Id,
    name: String,
    description: Option<String>,
}

impl From<ShopFragment> for Shop {
    fn from(f: ShopFragment) -> Self {
        Self {
            id: f.id.into_inner(),
            name: f.name,
            description: f.description,
        }
    }
}

/// Fetches basic shop information.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "ShopQueryVariables")]
pub struct ShopQuery {
    shop: ShopFragment,
}

impl ShopQuery {
    pub fn build(vars: ShopQueryVariables) -> cynic::Operation<Self, ShopQueryVariables> {
        <Self as cynic::QueryBuilder<ShopQueryVariables>>::build(vars)
    }
}

impl From<ShopQuery> for Shop {
    fn from(q: ShopQuery) -> Self {
        q.shop.into()
    }
}

// ── Products query ────────────────────────────────────────────────────────────

/// Variables for the [`ProductsQuery`].
#[derive(cynic::QueryVariables, Debug, Clone)]
pub struct ProductsQueryVariables {
    pub first: i32,
}

/// Fetches a paginated list of products.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "ProductsQueryVariables")]
pub struct ProductsQuery {
    #[arguments(first: $first)]
    products: ProductConnectionFragment,
}

impl ProductsQuery {
    pub fn build(vars: ProductsQueryVariables) -> cynic::Operation<Self, ProductsQueryVariables> {
        <Self as cynic::QueryBuilder<ProductsQueryVariables>>::build(vars)
    }
}

impl From<ProductsQuery> for ProductConnection {
    fn from(q: ProductsQuery) -> Self {
        Self {
            nodes: q.products.nodes.into_iter().map(Product::from).collect(),
        }
    }
}

// ── Product query ─────────────────────────────────────────────────────────────

/// Variables for the [`ProductQuery`].
#[derive(cynic::QueryVariables, Debug, Clone)]
pub struct ProductQueryVariables {
    pub handle: String,
}

/// Fetches a single product by handle.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "ProductQueryVariables")]
pub struct ProductQuery {
    #[arguments(handle: $handle)]
    product: Option<ProductFragment>,
}

impl ProductQuery {
    pub fn build(vars: ProductQueryVariables) -> cynic::Operation<Self, ProductQueryVariables> {
        <Self as cynic::QueryBuilder<ProductQueryVariables>>::build(vars)
    }
}

impl From<ProductQuery> for Option<Product> {
    fn from(q: ProductQuery) -> Self {
        q.product.map(Product::from)
    }
}

// ── Collections query ─────────────────────────────────────────────────────────

/// Variables for the [`CollectionsQuery`].
#[derive(cynic::QueryVariables, Debug, Clone)]
pub struct CollectionsQueryVariables {
    pub first: i32,
}

/// Fetches a paginated list of collections.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "CollectionsQueryVariables")]
pub struct CollectionsQuery {
    #[arguments(first: $first)]
    collections: CollectionConnectionFragment,
}

impl CollectionsQuery {
    pub fn build(
        vars: CollectionsQueryVariables,
    ) -> cynic::Operation<Self, CollectionsQueryVariables> {
        <Self as cynic::QueryBuilder<CollectionsQueryVariables>>::build(vars)
    }
}

impl From<CollectionsQuery> for CollectionConnection {
    fn from(q: CollectionsQuery) -> Self {
        Self {
            nodes: q
                .collections
                .nodes
                .into_iter()
                .map(Collection::from)
                .collect(),
        }
    }
}

// ── Collection query ──────────────────────────────────────────────────────────

/// Variables for the [`CollectionQuery`].
#[derive(cynic::QueryVariables, Debug, Clone)]
pub struct CollectionQueryVariables {
    pub handle: Option<String>,
    pub id: Option<cynic::Id>,
}

/// Fetches a single collection by handle or ID.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "CollectionQueryVariables")]
pub struct CollectionQuery {
    #[arguments(handle: $handle, id: $id)]
    collection: Option<CollectionFragment>,
}

impl CollectionQuery {
    pub fn build(
        vars: CollectionQueryVariables,
    ) -> cynic::Operation<Self, CollectionQueryVariables> {
        <Self as cynic::QueryBuilder<CollectionQueryVariables>>::build(vars)
    }
}

impl From<CollectionQuery> for Option<Collection> {
    fn from(q: CollectionQuery) -> Self {
        q.collection.map(Collection::from)
    }
}
