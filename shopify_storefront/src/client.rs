use crate::{API_VERSION, error::Error};
use cynic::{GraphQlResponse, Operation};
use serde::de::DeserializeOwned;

/// Client for the Shopify Storefront API.
///
/// Construct with [`Client::new`], optionally set the access token via
/// [`Client::token`], then execute queries with [`Client::run`].
///
/// ```no_run
/// let client = Client::new("my-store.myshopify.com");
/// ```
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    endpoint: String,
    token: String,
}

impl Client {
    /// Creates a new client for the given shop domain.
    pub fn new(domain: impl Into<String>) -> Self {
        let domain = domain.into();
        let endpoint = format!("https://{}/api/{}/graphql.json", domain, API_VERSION);

        Self {
            http: reqwest::Client::default(),
            endpoint,
            token: String::new(),
        }
    }

    /// Creates a client from environment variables.
    ///
    /// - `SHOPIFY_DOMAIN` — required. The shop domain, e.g. `mock.shop`.
    /// - `SHOPIFY_SF_TOKEN` — optional. The public Storefront access token.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingEnvVar`] if `SHOPIFY_DOMAIN` is not set.
    pub fn from_env() -> Result<Self, crate::Error> {
        let domain = std::env::var("SHOPIFY_DOMAIN")
            .map_err(|_| crate::Error::MissingEnvVar("SHOPIFY_DOMAIN"))?;

        let token = std::env::var("SHOPIFY_SF_TOKEN").unwrap_or_default();

        let mut client = Self::new(domain);
        client.token(token);
        Ok(client)
    }

    /// Sets the Storefront API access token.
    pub fn token(&mut self, token: impl Into<String>) {
        self.token = token.into();
    }

    /// Executes a typed cynic [`Operation`] and returns the deserialized response data.
    pub async fn run<ResponseData, Vars>(
        &self,
        operation: Operation<ResponseData, Vars>,
    ) -> Result<ResponseData, Error>
    where
        ResponseData: DeserializeOwned + 'static,
        Vars: serde::Serialize,
    {
        let response: GraphQlResponse<ResponseData> = self
            .http
            .post(&self.endpoint)
            .header("X-Shopify-Storefront-Access-Token", &self.token)
            .json(&operation)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        match response.errors {
            Some(errors) if !errors.is_empty() => {
                let mapped = errors
                    .into_iter()
                    .map(|e| crate::error::GraphqlError {
                        message: e.message,
                        extensions: None,
                    })
                    .collect();
                Err(Error::GraphQL(mapped))
            }
            _ => response.data.ok_or_else(|| {
                Error::GraphQL(vec![crate::error::GraphqlError {
                    message: "Response contained no data".into(),
                    extensions: None,
                }])
            }),
        }
    }
}
