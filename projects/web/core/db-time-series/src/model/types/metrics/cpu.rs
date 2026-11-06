use crate::model::types::metrics::metric::ToRawQuery;

pub enum CpuQuery {
    CpuUsage { host: String }
}

impl ToRawQuery for CpuQuery {
    fn to_metricql(&self) -> String {
        match self {
            CpuQuery::CpuUsage { host } => {
                format!("cpu_usage{{host='{}'}}", host)
            }
        }
    }
}