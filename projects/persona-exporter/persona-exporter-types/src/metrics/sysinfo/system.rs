use crate::metrics::structs::common::DiskUsage;
use crate::metrics::structs::system::LoadAverage;
use sysinfo::LoadAvg;

impl From<LoadAvg> for LoadAverage {
    fn from(value: LoadAvg) -> Self {
        Self {
            one: value.one,
            five: value.five,
            fifteen: value.fifteen,
        }
    }
}

impl From<sysinfo::DiskUsage> for DiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        Self {
            read_bytes: value.read_bytes,
            written_bytes: value.written_bytes,
            total_read_bytes: value.total_read_bytes,
            total_written_bytes: value.total_written_bytes,
        }
    }
}
