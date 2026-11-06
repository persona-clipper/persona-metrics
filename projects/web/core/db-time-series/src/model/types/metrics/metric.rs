use crate::model::types::metrics::cpu::CpuQuery;
use crate::model::types::metrics::memory::MemoryQuery;

pub trait ToRawQuery {
    fn to_metricql(&self) -> String;
}

pub enum MetricQuery {
    Cpu(CpuQuery),
    Memory(MemoryQuery)
}

impl ToRawQuery for MetricQuery {
    fn to_metricql(&self) -> String {
        match self {
            MetricQuery::Cpu(_) => {}
            MetricQuery::Memory(_) => {}
        }
    }
}