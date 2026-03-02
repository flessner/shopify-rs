/// Shopify Storefront API client.
///
/// # Schema
///
/// The vendored GraphQL schema lives at `schemas/storefront.graphql`.
/// Reference it in your `cynic::use_schema!` or `#[cynic::schema(...)]`
/// attributes to derive strongly-typed query fragments.
mod client;
mod error;

pub use client::Client;
pub use error::{Error, GraphqlError};

/// The Shopify Storefront API version this crate targets.
pub const API_VERSION: &str = "2026-01";
