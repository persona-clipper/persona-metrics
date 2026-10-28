/// Details system memory information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct MemoryInfo {
    /// Total amount RAM memory in your system
    pub total_memory: u64,
    /// Used amount RAM memory in your system
    pub used_memory: u64,
    /// Free and physically accessible memory
    pub free_memory: u64,
    /// Available memory that the system can allocate to a
    /// program without compromising the OS.
    pub available_memory: u64,
    /// Total paging file size, see `total_memory`
    pub total_swap: u64,
    /// Used page file size, see `used_memory`
    pub used_swap: u64,
    /// Free and physically accessible swap, see `free_memory`
    pub free_swap: u64,
}
