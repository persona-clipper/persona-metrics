use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::server::SendInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&SendInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SendInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("url", &value.url)
            .tag("server_name", &value.server_name)
            .field("", 0.0)
    }
}
