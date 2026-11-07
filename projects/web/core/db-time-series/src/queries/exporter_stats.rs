use std::fmt::Write;
use crate::types::metrics_query::ToRawQuery;

pub enum ExporterStatsQuery {
    GlobalCpuUsage { host: String },
    MemoryUsage { host: String }
}

impl ToRawQuery for ExporterStatsQuery {
    fn to_promql(&self, buffer: &mut String) {
        match self {
            Self::GlobalCpuUsage { host } => {
                let _ = write!(buffer, "metrics_exporter_monitoring_cpu_usage.global{{hostname='{}'}}", host);
            }
            Self::MemoryUsage { host} => {
                let _ = write!(buffer, "metrics_exporter_monitoring_memory_usage{{hostname='{}'}}", host);
            }
        }
    }
}