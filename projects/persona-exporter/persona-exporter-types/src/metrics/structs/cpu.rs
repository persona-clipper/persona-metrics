use crate::metrics::traits::Clear;
use compact_str::CompactString;
use smallvec::SmallVec;

/// Statistics on the machine's processor
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct CpuListInfo {
    /// System CPU usage. Measured as a percentage from 0%-100%
    pub global_cpu_usage: f64,
    /// Total number of logical threads
    pub threads: usize,
    /// Number of physical processor cores
    pub physical_core_count: usize,
    pub cpu_cores: SmallVec<[CpuThreadInfo; 8]>,
}

/// Information about a single logical processor thread
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct CpuThreadInfo {
    /// The name of the logical thread used in the operating system.
    pub os_name: CompactString,
    /// Processor logical thread clock frequency in MHz.
    pub frequency: u64,
    /// Processor time usage percentage relative to a single thread
    pub cpu_usage: f32,
}

impl Clear for CpuListInfo {
    fn clear_dynamic(&mut self) {
        self.cpu_cores.clear();

        // self.cpu_usage = 0.0;
        // self.threads = 0;
        // self.physical_core_count = 0;
    }
}
