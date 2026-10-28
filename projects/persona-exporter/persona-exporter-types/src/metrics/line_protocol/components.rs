use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::components::ComponentInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&ComponentInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &ComponentInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("id", &value.id)
            .tag("name", &value.name)
            .field("temp", value.temp as i64)
            .field("critical_temp", value.critical_temp as i64)
            .field("max_temp", value.max_temp as i64)
    }
}
