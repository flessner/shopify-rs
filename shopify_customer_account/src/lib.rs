/// Shopify Customer Account API client.
///
/// # Schema
///
/// The vendored GraphQL schema lives at `schemas/customer_account.graphql`.
/// Reference it in your `#[cynic::schema("customer_account")]` attribute to
/// derive strongly-typed query fragments.
///
/// # Authentication
///
/// The Customer Account API uses OAuth 2.0 with PKCE. This crate provides only
/// the GraphQL client — obtaining and refreshing tokens is the caller's
/// responsibility. Pass the access token directly to [`Client::run`].
mod client;
mod error;

pub use client::{Client, CustomerAccountConfig, OpenIdConfig};
pub use error::{Error, GraphqlError};
