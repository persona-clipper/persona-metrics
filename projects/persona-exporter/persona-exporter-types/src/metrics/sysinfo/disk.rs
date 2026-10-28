use crate::metrics::structs::common::DiskUsage;
use crate::metrics::structs::disk::StorageMountPointInfo;
use compact_str::CompactString;
use sysinfo::Disk;

impl From<&sysinfo::Disk> for StorageMountPointInfo {
    fn from(value: &Disk) -> Self {
        Self {
            name: CompactString::from(value.name().to_string_lossy()),
            mount_point: CompactString::from(value.mount_point().to_string_lossy()),
            file_system: CompactString::from(value.file_system().to_string_lossy()),
            total_space: value.total_space(),
            kind: CompactString::from(value.kind().to_string()),
            available_space: value.available_space(),
            used_space: value.total_space() - value.available_space(),
            disk_usage: DiskUsage::from(value.usage()),
            is_read_only: value.is_read_only(),
            is_removable: value.is_removable(),
        }
    }
}
