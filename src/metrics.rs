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
/// # Example
/// ```text
/// ```
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
    pub kernel_version: String,
    pub kernel_long_version: String,
    pub distribution_id: String,
    pub distribution_id_like: Vec<String>,
    pub boot_time: u64,
    pub uptime: u64,
    pub cpu_arch: String,
    pub os_version: String,
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
    pub available_memory: u64,

    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,

    /// Metric showing the average load on processor threads
    pub load_avg: LoadAverage,
}

///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub file_system: String,
    pub kind: String,
    pub total_space: u64,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentsInfo {
    pub count: usize,
    pub is_empty: bool,
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

/// Processor thread information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentInfo {
    ///
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
