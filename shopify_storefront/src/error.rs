use thiserror::Error;

/// A GraphQL error returned by the Shopify Storefront API.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphqlError {
    pub message: String,
    #[serde(default)]
    pub extensions: Option<serde_json::Value>,
}

impl std::fmt::Display for GraphqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Errors that can occur when interacting with the Shopify Storefront API.
#[derive(Debug, Error)]
pub enum Error {
    /// An HTTP-level error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned one or more GraphQL errors.
    #[error("GraphQL errors: {}", format_errors(.0))]
    GraphQL(Vec<GraphqlError>),

    /// A required environment variable was missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(&'static str),
}

fn format_errors(errors: &[GraphqlError]) -> String {
    errors
        .iter()
        .map(|e| e.message.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
