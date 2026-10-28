use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::structs::common::DiskUsage;
use crate::metrics::traits::Clear;
use compact_str::CompactString;
use core::fmt;
use std::fmt::Formatter;

/// System process information: top N processes and information from the exporter itself.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ProcessListInfo {
    /// Exporter information
    pub exporter_metrics: Option<ProcessInfo>,
    /// Information on processes
    pub process_list: Vec<ProcessInfo>,
}

impl Default for ProcessListInfo {
    fn default() -> Self {
        Self {
            exporter_metrics: Some(ProcessInfo::default()),
            process_list: Vec::with_capacity(5),
        }
    }
}

/// Information about the process
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub name: CompactString,

    /// Process status at the time of recording. See also [`ProcessStatus`]
    pub status: ProcessStatus,
    /// Disk space usage information. See also [`DiskUsage`]
    pub disk_usage: DiskUsage,

    /// Unique process identifier
    pub program_id: CompactString,

    /// Information about process usage (usually as a percentage) at a given point in time
    pub cpu_usage_per_thread: f32,
    pub global_cpu_usage: f32,
    /// Information about RAM usage (in bytes or another unit of measurement) at this moment in time
    pub memory_usage: u64,
    /// Information on swap file usage
    pub virtual_memory: u64,
    /// Process Lifespan in seconds
    pub run_time: u64,
    /// Displays the time the process was started in UNIX time
    pub start_time: u64,
    /// User ID of the user who started the process
    pub user_id: CompactString,
    /// ID of the group that initiated the process
    pub group_id: CompactString,
}

/// Represents the current execution state of a process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// The state of the process cannot be determined.
    #[default]
    Unknown,
    /// The process is actively running on a CPU core.
    Run,
    /// The process is idle and waiting for work (common on BSD).
    Idle,
    /// The process finished execution but its parent has not read its exit code yet.
    Zombie,
    /// The process is sleeping and waiting for an event or signal.
    Sleep,
    /// The process is being inspected by a debugger or tracing tool.
    Tracing,
    /// The process was stopped by a signal (like SIGSTOP).
    Stop,
    /// The process is dead and completely terminated.
    Dead,
    /// The process is in a deep sleep but will wake up to handle a fatal signal.
    Wakekill,
    /// The process is currently moving from sleep to a running state.
    Waking,
    /// The process thread is parked (usually for internal kernel management).
    Parked,
    /// The process is blocked waiting for a lock.
    LockBlocked,
    /// The process is in a deep sleep waiting for disk Input/Output operations.
    UninterruptibleDiskSleep,
    /// The process is suspended or paused (common on macOS).
    Suspended,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Default for ProcessInfo {
    fn default() -> Self {
        Self {
            name: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            status: ProcessStatus::default(),
            disk_usage: DiskUsage::default(),
            program_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            cpu_usage_per_thread: 0.0,
            memory_usage: 0,
            virtual_memory: 0,
            run_time: 0,
            start_time: 0,
            user_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            group_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            global_cpu_usage: 0.0,
        }
    }
}

impl Clear for ProcessListInfo {
    fn clear_dynamic(&mut self) {
        // self.exporter_metrics;
        // self.exporter_metrics.get_or_insert_default().clear_dynamic();
        self.process_list.clear();
    }
}

impl Clear for ProcessInfo {
    fn clear_dynamic(&mut self) {
        self.name.clear();
        self.program_id.clear();
        self.user_id.clear();
        self.group_id.clear();

        // self.status = ProcessStatus::Unknown;
        // self.disk_usage.clear();
    }
}
