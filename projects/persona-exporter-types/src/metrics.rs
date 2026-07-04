/// General structure that contains all the metrics of the working machine
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ServerMetrics {
    /// Display system information
    pub system: Option<SystemInfo>,
    /// RAM memory information
    pub memory: Option<MemoryInfo>,
    /// Disk indicators
    pub disk: Option<DiskInfo>,
    /// Network indicators
    pub network: Option<NetworkInfo>,
    /// CPU indicators
    pub cpu: Option<CpuInfo>,
    /// Components indicators
    pub components: Option<ComponentsInfo>,
    /// Time in UNIX when the metrics were recorded
    pub timestamp: u64,
}
/// General system information: kernel version, username,
/// uptime, processor architecture, and so on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct SystemInfo {
    /// Name of your operating system.
    ///
    /// | PLATFORM | NAME |
    /// | --- | --- |
    /// | laptop with **Linux** | "NixOS" |
    /// | PC with **Windows** | "Windows" |
    pub name: String,
    /// kernel version of your OS
    pub kernel_version: String,
    /// kernel version + system name
    pub kernel_long_version: String,
    /// Unique your distribution ID
    pub distribution_id: String,
    /// The family to which your distribution belongs.
    /// for example, if you are using Ubuntu, the field
    /// value will be `["debian"]`, since Ubuntu is a derivative of Debian.
    pub distribution_id_like: Vec<String>,
    /// The UNIX time at which the system booted
    pub boot_time: u64,
    /// System uptime
    pub uptime: u64,
    /// Your processor architecture
    pub cpu_arch: String,
    /// Your OS version
    pub os_version: String,
    /// Your hostname
    pub host_name: String,
}
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

    /// Metric showing the average load on processor threads
    pub load_avg: LoadAverage,
}

/// System disk space information for the root directory "/".
/// There is no breakdown by physical storage devices.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct DiskInfo {
    /// Disk name
    pub name: String,
    /// File system, line "ext4", "btrfs"
    pub file_system: String,
    /// Disk kind: HDD / SDD etc.
    pub kind: String,
    /// Total space
    pub total_space: u64,
    /// Available space
    pub available_space: u64,
}

/// Network information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct NetworkInfo {
    /// The name of your card's network interface
    pub interface_name: String,

    /// Total bytes received since the network card was turned on
    pub total_rx_bytes: u64,
    /// Total data packets received
    pub total_rx_packets: u64,
    /// Total errors when accepting data
    pub total_rx_errors: u64,

    /// Total bytes transferred since the network card was turned on
    pub total_tx_bytes: u64,
    /// Total data packets transferred
    pub total_tx_packets: u64,
    /// Total errors when sending data
    pub total_tx_errors: u64,
}
/// The component information primarily consists of data regarding the temperatures of
/// individual components (circuit boards, processor cores, etc.).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentsInfo {
    /// Components count
    pub count: usize,
    /// Checks whether the component field is empty.
    pub is_empty: bool,
    /// Components info, see [`ComponentInfo`]
    pub components: Vec<ComponentInfo>,
}
/// Statistics on the machine's processor
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct CpuInfo {
    /// System CPU usage. Measured as a percentage from 0%-100%
    pub cpu_usage: f32,
    /// Number of processor threads
    pub threads: usize,
    /// Number of physical processor cores
    pub physical_core_count: usize,
}

/// Processor thread information, used in [`ComponentsInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentInfo {
    /// Component identifier recognized by the system kernel
    pub id: String,
    /// Component name
    pub name: String,
    /// Component temp
    pub temp: f32,
    /// Critical temp
    pub critical_temp: f32,
    /// Max temp of component
    pub max_temp: f32,
}

/// Load Average structure for `load_avg` field in [CpuInfo]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct LoadAverage {
    /// LA at one minute
    pub one: f32,
    /// LA at five minutes
    pub five: f32,
    /// LA at fifteen minutes
    pub fifteen: f32,
}
