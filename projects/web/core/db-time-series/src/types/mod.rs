use serde::{Deserialize, Serialize};

pub mod metrics_query;
pub mod client;
pub mod step;
pub mod response;


#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub step: Option<String>,
}