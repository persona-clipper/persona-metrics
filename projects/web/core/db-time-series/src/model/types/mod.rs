pub mod metrics;

use serde_json::json;
use url::Url;
use crate::model::types::metrics::metric::{MetricQuery, ToRawQuery};

pub struct VictoriaMetrics {
    pub client: reqwest::Client,
    pub url: url::Url,
}

/// В секундах
pub struct Step(pub i32);

pub enum StepRange {
    Seconds,
    Minutes,
    Hours,
}

impl StepRange {
    pub fn to_str(&self) -> &'static str {
        match &self {
            StepRange::Seconds => "s",
            StepRange::Minutes => "m",
            StepRange::Hours => "h",
        }
    }
}
impl Step {
    pub fn to_metrics_step(&self, range: StepRange) -> String {
        format!("{}{}", self.0, range.to_str())
    }
}

impl VictoriaMetrics {
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

    pub async fn query(&self, query: MetricQuery, step: &str,) -> Result<serde_json::Value, Box<dyn std::error::Error>>{
        let raw_query = query.to_promql();

        let result = self.raw_query(&raw_query, step).await?;

        Ok(result)
    }

    pub async fn query_range(&self, query: MetricQuery, start: i64, end: i64, step: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let raw_query = query.to_promql();

        let result = self.raw_query_range(&raw_query, start, end, step).await?;

        Ok(result)
    }
}