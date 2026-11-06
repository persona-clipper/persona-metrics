use crate::model::types::metrics::metric::ToRawQuery;

pub enum MemoryQuery {
    MemoryUsed { host: String }
}

impl ToRawQuery for MemoryQuery {
    fn to_promql(&self) -> String {
        match self {
            MemoryQuery::MemoryUsed { host } => {
                format!("metrics_memory_used_memory{{hostname='{}'}}", host)
            }
        }
    }
}