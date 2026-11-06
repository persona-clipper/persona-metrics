use crate::model::types::metrics::{CpuQuery, MemoryQuery, ProcessListQuery};

pub trait ToRawQuery {
    fn to_promql(&self) -> String;
}

pub enum MetricQuery {
    Cpu(CpuQuery),
    Memory(MemoryQuery),
    ProcessList(ProcessListQuery),
}

impl ToRawQuery for MetricQuery {
    fn to_promql(&self) -> String {
        match self {
            MetricQuery::Cpu(query) => query.to_promql(),
            MetricQuery::Memory(query) => query.to_promql(),
            MetricQuery::ProcessList(query) => query.to_promql(),
        }
    }
}