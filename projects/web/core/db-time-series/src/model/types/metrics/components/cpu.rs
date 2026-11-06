use crate::model::types::metrics::metric::ToRawQuery;

pub enum CpuQuery {
    CpuUsage { host: String }
}

impl ToRawQuery for CpuQuery {
    fn to_promql(&self) -> String {
        match self {
            CpuQuery::CpuUsage { host } => {
                format!("metrics_cpu_global_cpu_usage{{hostname='{}'}}", host)
            }
        }
    }
}