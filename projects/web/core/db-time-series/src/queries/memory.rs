use std::fmt::Write;
use crate::types::metrics_query::ToRawQuery;

pub enum MemoryQuery {
    MemoryUsed { host: String }
}

impl ToRawQuery for MemoryQuery {
    fn to_promql(&self, buffer: &mut String) {
        match self {
            Self::MemoryUsed { host } => {
                let _ = write!(buffer, "metrics_memory_used_memory{{hostname='{}'}}", host);
            }
        }
    }
}