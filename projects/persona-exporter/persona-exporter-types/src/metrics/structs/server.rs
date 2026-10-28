use compact_str::CompactString;

/// General structure that contains all the metrics of the working machine
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ServerMetrics {
    #[cfg(feature = "system")]
    /// Display system information
    pub system: Option<crate::metrics::structs::system::SystemInfo>,
    #[cfg(feature = "processes")]
    /// Display process list
    pub process_list: Option<crate::metrics::structs::processes::ProcessListInfo>,
    #[cfg(feature = "memory")]
    /// RAM memory information
    pub memory: Option<crate::metrics::structs::memory::MemoryInfo>,
    #[cfg(feature = "disk")]
    /// Disk indicators
    pub disk: Option<crate::metrics::structs::disk::StorageListInfo>,
    /// Network indicators
    #[cfg(feature = "network")]
    pub network: Option<crate::metrics::structs::network::NetworkInfo>,
    #[cfg(feature = "cpu")]
    /// CPU indicators
    pub cpu: Option<crate::metrics::structs::cpu::CpuListInfo>,
    #[cfg(feature = "components")]
    /// Components indicators
    pub components: Option<crate::metrics::structs::components::ComponentListInfo>,

    /// Metric showing the average load on processor threads
    // pub load_average: Option<LoadAverage>,
    /// Time in UNIX when the metrics were recorded
    pub time: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct SendInfo {
    pub url: CompactString,
    pub server_name: CompactString,
}
