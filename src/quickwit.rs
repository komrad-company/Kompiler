use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<Value>,
    pub num_hits: u64,
    pub elapsed_time_micros: u64,
}

pub struct QuickwitClient {
    http: reqwest::Client,
    base_url: String,
}

impl QuickwitClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    // Recherche via l'API native Quickwit
    pub async fn search(
        &self,
        index: &str,
        query: &str,
        max_hits: u64,
    ) -> anyhow::Result<SearchResponse> {
        let url = format!("{}/api/v1/{}/search", self.base_url, index);
        let body = json!({
            "query": query,
            "max_hits": max_hits,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<SearchResponse>()
            .await?;

        Ok(resp)
    }
}
