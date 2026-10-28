/// Represents the statistics of disk activity for a specific process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DiskUsage {
    /// Number of bytes read from disk since the last update.
    pub read_bytes: u64,
    /// Number of bytes written to disk since the last update.
    pub written_bytes: u64,
    /// Total number of bytes read from disk since the process started.
    pub total_read_bytes: u64,
    /// Total number of bytes written to disk since the process started.
    pub total_written_bytes: u64,
}
