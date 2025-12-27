use serde::{Deserialize, Serialize};

pub struct StorefrontClient {
    pub domain: String,
    pub version: String,
    pub token: String,
}

#[derive(Clone, Deserialize)]
struct StorefrontResponse {
    data: StorefrontData,
}

#[derive(Clone, Deserialize)]
pub struct StorefrontData {
    pub shop: Option<StorefrontShop>,
    pub collections: Option<Connection<StorefrontCollection>>,
    pub collection: Option<StorefrontCollection>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StorefrontShop {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct StorefrontCollection {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Deserialize)]
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
}

#[derive(Clone, Deserialize)]
pub struct Edge<T> {
    pub node: T,
}

impl StorefrontClient {
    pub fn new(domain: String, token: String) -> StorefrontClient {
        return StorefrontClient {
            version: "2025-10".to_string(),
            domain,
            token,
        };
    }

    pub async fn fetch(&self, query: &str) -> Result<StorefrontData, Box<dyn std::error::Error>> {
        let url = format!("https://{}/api/{}/graphql.json", self.domain, self.version);
        let payload = serde_json::json!({"query": query});

        #[cfg(not(target_family = "wasm"))]
        {
            let client = reqwest::Client::new();
            let response: StorefrontResponse = client
                .post(&url)
                .header("X-Shopify-Storefront-Access-Token", &self.token)
                .json(&payload)
                .send()
                .await?
                .json()
                .await?;

            return Ok(response.data);
        }

        #[cfg(target_family = "wasm")]
        {
            let response: StorefrontResponse = gloo_net::http::Request::post(&url)
                .header("X-Shopify-Storefront-Access-Token", &self.token)
                .json(&payload)?
                .send()
                .await?
                .json()
                .await?;

            return Ok(response.data);
        }
    }

    pub async fn fetch_shop(&self) -> Result<StorefrontShop, Box<dyn std::error::Error>> {
        let query = r#"
            query {
                shop {
                    id
                    name
                    description
                }
            }"#;

        let shop = self
            .fetch(query)
            .await?
            .shop
            .ok_or("No shop data in response")?;

        return Ok(shop);
    }

    pub async fn fetch_collections(
        &self,
    ) -> Result<Vec<StorefrontCollection>, Box<dyn std::error::Error>> {
        let query = r#"
            query {
                collections(first: 250) {
                    edges {
                        node {
                            id
                            title
                            description
                        }
                    }
                }
            }"#;

        let collections = self
            .fetch(query)
            .await?
            .collections
            .ok_or("No collections data in response")?
            .edges
            .into_iter()
            .map(|e| e.node)
            .collect();

        return Ok(collections);
    }
}
