use crate::DEFAULT_EMPTY_MESSAGE;
use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::system::SystemInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use compact_str::CompactString;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&SystemInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SystemInfo, measurement: &str, global_tags: &GlobalTags) -> Self {
        let mut distro_like = CompactString::from(value.distribution_id_like.join(","));
        if distro_like.is_empty() {
            distro_like = CompactString::from(DEFAULT_EMPTY_MESSAGE);
        }

        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("name", &value.name)
            .tag("kernel_version", &value.kernel_version)
            .tag("kernel_long_version", &value.kernel_long_version)
            .tag("distribution_id", &value.distribution_id)
            .tag("distribution_id_like", distro_like.as_str())
            .tag("cpu_arch", &value.cpu_arch)
            .tag("os_version", &value.os_version)
            .tag("host_name", &value.host_name)
            .field("boot_time", value.boot_time)
            .field("uptime", value.uptime)
            .field("load_average.one", value.load_average.one)
            .field("load_average.five", value.load_average.five)
            .field("load_average.fifteen", value.load_average.fifteen)
    }
}
