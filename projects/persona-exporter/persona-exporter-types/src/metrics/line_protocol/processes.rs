use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::processes::ProcessInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&ProcessInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &ProcessInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        let status = value.status.to_string();
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("name", &value.name)
            .tag("user_id", &value.user_id)
            .tag("group_id", &value.group_id)
            .tag("program_id", &value.program_id)
            .field("status", &*status)
            .field("disk_usage.read_bytes", value.disk_usage.read_bytes)
            .field("disk_usage.written_bytes", value.disk_usage.written_bytes)
            .field(
                "disk_usage.total_read_bytes",
                value.disk_usage.total_read_bytes,
            )
            .field(
                "disk_usage.total_written_bytes",
                value.disk_usage.total_written_bytes,
            )
            .field("cpu_usage.per_thread", f64::from(value.cpu_usage_per_thread))
            .field("cpu_usage.global", f64::from(value.global_cpu_usage))
            .field("memory_usage", value.memory_usage)
            .field("virtual_memory", value.virtual_memory)
            .field("run_time", value.run_time)
            .field("start_time", value.start_time)
    }
}
