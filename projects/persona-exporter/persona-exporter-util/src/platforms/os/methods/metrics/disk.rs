use persona_exporter_types::metrics::{structs::disk::{StorageListInfo, StorageMountPointInfo}, traits::Clear};
use sysinfo::Disks;

pub fn collect_storage_list_metrics(disks: &mut Disks, disk_buffer: &mut StorageListInfo) {
    for disk in disks.list() {
        disk_buffer
            .storage_list
            .push(StorageMountPointInfo::from(disk));
    }
}

// pub fn collect_storage_info(mount_points: Vec<StorageMountPointInfo>) -> StorageInfo {
//   StorageInfo {
//       mount_points: mount_points,
//       name: Default::default(),
//       kind: Default::default(),
//   }
// }

pub fn try_collect_disk_metrics(
    disks_snapshot: &mut Option<Disks>,
    disk_buffer: &mut Option<StorageListInfo>,
) {
    if let (Some(d), Some(buf)) = (disks_snapshot, disk_buffer) {
        d.refresh(false);
        buf.clear_dynamic();
        collect_storage_list_metrics(d, buf);
    }
}

