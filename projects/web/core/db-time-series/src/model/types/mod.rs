pub mod metrics;

use serde_json::json;
use url::Url;

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

        let query = [
            ("query", query),
            ("step", step),
        ];
        let timestamp = [
            ("start", start),
            ("end", end),
        ];

        let json_response = self.client
            .get(url.to_string())
            .query(&query)
            .query(&timestamp)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(json_response)
    }
}