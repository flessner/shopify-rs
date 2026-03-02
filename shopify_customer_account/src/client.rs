use crate::error::Error;
use cynic::{GraphQlResponse, Operation};
use serde::de::DeserializeOwned;
use tokio::sync::OnceCell;

/// The OpenID Connect configuration discovered from
/// `/.well-known/openid-configuration`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct OpenIdConfig {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub end_session_endpoint: String,
    pub jwks_uri: String,
}

/// The Customer Account API configuration discovered from
/// `/.well-known/customer-account-api`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CustomerAccountConfig {
    pub graphql_api: String,
    pub mcp_api: String,
}

/// Client for the Shopify Customer Account API.
///
/// Construct with [`Client::new`], passing the shop domain. The GraphQL
/// endpoint is discovered automatically from
/// `https://{shop_domain}/.well-known/customer-account-api` on the first
/// call to [`Client::run`] and cached for all subsequent calls.
///
/// ```no_run
/// let client = Client::new("my-store.myshopify.com");
/// ```
pub struct Client {
    http: reqwest::Client,
    domain: String,
    endpoint: OnceCell<String>,
}

impl Client {
    /// Creates a new client for the given shop domain.
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::default(),
            domain: domain.into(),
            endpoint: OnceCell::new(),
        }
    }

    /// Fetches the OpenID Connect configuration from
    /// `/.well-known/openid-configuration`.
    pub async fn fetch_openid_config(&self) -> Result<OpenIdConfig, Error> {
        Ok(self
            .http
            .get(format!(
                "https://{}/.well-known/openid-configuration",
                self.domain
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Fetches the Customer Account API configuration from
    /// `/.well-known/customer-account-api`.
    pub async fn fetch_customer_account_config(&self) -> Result<CustomerAccountConfig, Error> {
        Ok(self
            .http
            .get(format!(
                "https://{}/.well-known/customer-account-api",
                self.domain
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Executes a typed cynic [`Operation`] with the given customer access token.
    ///
    /// On the first call the GraphQL endpoint is discovered via
    /// `/.well-known/customer-account-api` and cached for subsequent calls.
    /// The token is sent as `Authorization: <token>` per the Customer Account
    /// API spec.
    pub async fn run<ResponseData, Vars>(
        &self,
        operation: Operation<ResponseData, Vars>,
        token: &str,
    ) -> Result<ResponseData, Error>
    where
        ResponseData: DeserializeOwned + 'static,
        Vars: serde::Serialize,
    {
        let endpoint = self
            .endpoint
            .get_or_try_init(|| async {
                let config = self.fetch_customer_account_config().await?;
                Ok::<String, Error>(config.graphql_api)
            })
            .await?;

        let response: GraphQlResponse<ResponseData> = self
            .http
            .post(endpoint)
            .header("Authorization", token)
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
