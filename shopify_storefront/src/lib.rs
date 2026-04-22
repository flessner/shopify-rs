/// Shopify Storefront API client.
///
/// # Schema
///
/// The vendored GraphQL schema lives at `schemas/storefront.graphql`.
mod client;
mod error;

pub mod queries;
pub mod types;

pub use client::Client;
pub use error::{Error, GraphqlError};

/// The Shopify Storefront API version this crate targets.
pub const API_VERSION: &str = "2026-01";
