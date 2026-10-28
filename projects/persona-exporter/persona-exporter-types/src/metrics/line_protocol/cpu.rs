use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::cpu::{CpuListInfo, CpuThreadInfo};
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&CpuThreadInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuThreadInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("os_name", &value.os_name)
            .field("frequency", value.frequency)
            .field("usage", f64::from(value.cpu_usage))
    }
}

impl FromWithMeasurement<&CpuListInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuListInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .field("global_cpu_usage", value.global_cpu_usage)
            .field("threads", value.threads as i64)
            .field("physical_core_count", value.physical_core_count as i64)
    }
}
