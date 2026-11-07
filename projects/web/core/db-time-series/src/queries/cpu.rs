use std::fmt::Write;
use crate::types::metrics_query::ToRawQuery;

pub enum CpuQuery {
    CpuUsage { host: String }
}

impl ToRawQuery for CpuQuery {
    fn to_promql(&self, buffer: &mut String) {
        match self {
            Self::CpuUsage { host } => {
                let _ = write!(buffer, "metrics_cpu_global_cpu_usage{{hostname='{}'}}", host);
            }
        }
    }
}