use crate::metrics::structs::common::DiskUsage;
use crate::metrics::traits::Clear;
use compact_str::CompactString;

/// System disk space information for the root directory "/".
/// There is no breakdown by physical storage devices.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct StorageMountPointInfo {
    /// Storage name, example: /dev/nvme0n1p1
    pub name: CompactString,
    /// Mount point
    pub mount_point: CompactString,
    /// File system, line "ext4", "btrfs"
    pub file_system: CompactString,
    /// Total space
    pub total_space: u64,
    /// Disk kind: HDD / SDD etc.
    pub kind: CompactString,
    /// Available space
    pub available_space: u64,
    /// Total space used by system
    pub used_space: u64,
    /// Disk usage
    pub disk_usage: DiskUsage,
    pub is_read_only: bool,
    pub is_removable: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct StorageListInfo {
    pub storage_list: Vec<StorageMountPointInfo>,
}

impl Clear for StorageListInfo {
    fn clear_dynamic(&mut self) {
        self.storage_list.clear();
    }
}
// impl Clear for StorageInfo {
//     fn clear_dynamic(&mut self) {
//         self.mount_points.clear();

// self.total_available_space = 0;
// self.total_available_space = 0;
// self.total_system_used = 0;
// }
// }

impl Clear for StorageMountPointInfo {
    fn clear_dynamic(&mut self) {
        self.file_system.clear();
        self.kind.clear();
        self.mount_point.clear();

        // self.total_space = 0;
        // self.available_space = 0;
    }
}
