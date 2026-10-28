use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::network::NetworkInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&NetworkInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &NetworkInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("interface_name", &value.interface_name)
            .field("total_rx_bytes", value.total_rx_bytes)
            .field("total_rx_packets", value.total_tx_packets)
            .field("total_rx_errors", value.total_rx_errors)
            .field("total_tx_bytes", value.total_tx_bytes)
            .field("total_tx_packets", value.total_tx_packets)
            .field("total_tx_errors", value.total_tx_errors)
    }
}
