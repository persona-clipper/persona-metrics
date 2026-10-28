use crate::metrics::line_protocol::GlobalTags;
use crate::metrics::structs::disk::StorageMountPointInfo;
use crate::traits::line_protocol::{FromWithMeasurement, InsertGlobalTags};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl FromWithMeasurement<&StorageMountPointInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(
        value: &StorageMountPointInfo,
        measurement: &str,
        global_tags: &GlobalTags,
    ) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .insert_global_tags(global_tags)
            .tag("mount_point", &value.mount_point)
            .tag("file_system", &value.file_system)
            .tag("kind", &value.kind)
            .tag("name", &value.name)
            // Крейт требует имеено &str / число. Не принимает bool
            .tag(
                "is_removable",
                if value.is_removable { "true" } else { "false" },
            )
            .tag(
                "is_read_only",
                if value.is_read_only { "true" } else { "false" },
            )
            .field("total_space", value.total_space)
            .field("available_space", value.available_space)
            .field("used_space", value.used_space)
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
    }
}

// impl FromWithMeasurement<&DiskInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
//     fn from_with_name(value: &DiskInfo, measurement: &str) -> Self {
//         LineProtocolBuilder::new()
//             .measurement(measurement)
//             .tag("name", unknown_or_value(&value.name))
//             .tag("file_system", unknown_or_value(&value.file_system))
//             .tag("kind", unknown_or_value(&value.kind))
//             .field("total_space", value.total_space)
//             .field("available_space", value.available_space)
//     }
// }

// impl FromWithMeasurement<&DiskListInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
//     fn from_with_name(value: &DiskListInfo, measurement: &str) -> Self {
//         LineProtocolBuilder::new()
//             .measurement(measurement)
//             .field("total_space", value.total_space)
//             .field("total_available_space", value.total_available_space)
//             .field("total_used_space", value.total_used)
//     }
// }
