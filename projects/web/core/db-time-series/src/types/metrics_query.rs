use std::fmt::Write;
use crate::queries::cpu::CpuQuery;
use crate::queries::memory::MemoryQuery;
use crate::queries::processes::ProcessListQuery;

pub trait ToRawQuery {
    fn to_promql(&self, buffer: &mut String);
}

pub enum MetricQuery {
    Cpu(CpuQuery),
    Memory(MemoryQuery),
    ProcessList(ProcessListQuery),
}

impl ToRawQuery for MetricQuery {
    fn to_promql(&self, mut buffer: &mut String) {
        match self {
            MetricQuery::Cpu(query) => query.to_promql(&mut buffer),
            MetricQuery::Memory(query) => query.to_promql(&mut buffer),
            MetricQuery::ProcessList(query) => query.to_promql(&mut buffer),
        }
    }
}