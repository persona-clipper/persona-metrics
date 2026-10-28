use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::memory::MemoryInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&MemoryInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &MemoryInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .field("total_memory", value.total_memory)
            .field("used_memory", value.used_memory)
            .field("free_memory", value.free_memory)
            .field("available_memory", value.available_memory)
            .field("total_swap", value.total_swap)
            .field("used_swap", value.used_swap)
            .field("free_swap", value.free_swap)
    }
}
