use url::Url;
use crate::types::metrics_query::{MetricQuery, ToRawQuery};

pub struct VictoriaClient {
    pub client: reqwest::Client,
    pub url: url::Url,
}

impl VictoriaClient {
    pub fn new(url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: url,
        }
    }

    pub async fn raw_query(&self, query: &str, step: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = self.url.join("api/v1/query")?;

        let queries = [
            ("query", query),
            ("step", step),
        ];

        let json_response= self
            .client
            .get(url.to_string())
            .query(&queries)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(json_response)
    }

    pub async fn raw_query_range(
        &self,
        query: &str,
        start: i64,
        end: i64,
        step: &str
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = self.url.join("api/v1/query_range")?;

        let start_str = start.to_string();
        let end_str = start.to_string();

        let query = [
            ("query", query),
            ("step", step),
            ("start", start_str.as_str()),
            ("end", end_str.as_str()),
        ];

        let json_response = self.client
            .get(url.to_string())
            .query(&query)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(json_response)
    }

    pub async fn query(&self, query: MetricQuery, step: &str, buffer: &mut String) -> Result<serde_json::Value, Box<dyn std::error::Error>>{
        buffer.clear();
        query.to_promql(buffer);

        let result = self.raw_query(&buffer, step).await?;

        Ok(result)
    }

    pub async fn query_range(&self, query: MetricQuery, start: i64, end: i64, step: &str, buffer: &mut String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        buffer.clear();
        query.to_promql(buffer);

        let result = self.raw_query_range(buffer, start, end, step).await?;

        Ok(result)
    }
}