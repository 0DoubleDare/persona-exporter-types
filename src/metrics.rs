use serde::{Deserialize, Serialize};

/// General structure that contains all the metrics of the working machine
#[derive(Serialize, Deserialize)]
pub struct ServerMetrics {
    pub sys_info: SystemInfo,
    pub disk_info: DiskInfo,
    pub network_data: NetworkInfo,
    pub cpu_info: CpuInfo,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: f32,
    pub physical_cpus_count: usize,
    pub threads: usize,
    pub load_average: LoadAverage,
}

///
#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    pub total_space: u64,
    pub available_space: u64,
}

/// Network information
#[derive(Serialize, Deserialize)]
pub struct NetworkInfo {
    /// The name of your card's network interface
    pub interface_name: String,

    /// Total bytes received since the network card was turned on
    pub total_rx_bytes: u64,
    /// Total data packets received
    pub total_rx_package: u64,
    /// Total errors when accepting data
    pub total_rx_errors: u64,

    /// Total bytes transferred since the network card was turned on
    pub total_tx_bytes: u64,
    /// Total data packets transferred
    pub total_tx_package: u64,
    /// Total errors when sending data
    pub total_tx_errors: u64,
}

/// Statistics on the machine's processor
#[derive(Serialize, Deserialize)]
pub struct CpuInfo {
    /// System CPU usage. Measured as a percentage from 0%-100%
    pub cpu_usage: f32,
    /// Metric showing the average load on processor threads
    pub load_avg: LoadAverage,
    /// Number of processor threads
    pub threads: usize,
    /// Number of physical processor cores
    pub physical_core_count: usize,
    /// Processor Thread Information
    pub components_info: Vec<ComponentInfo>,
}

/// Processor thread information
#[derive(Serialize, Deserialize)]
pub struct ComponentInfo {
    /// Component name
    pub name: String,
    /// Component temp
    pub temp: f64,
}

/// Load Average structure for `load_avg` field in [CpuInfo]
#[derive(Serialize, Deserialize)]
pub struct LoadAverage {
    /// LA at one minute
    pub one: f32,
    /// LA at five minutes
    pub five: f32,
    /// LA at fifteen minutes
    pub fifteen: f32,
}
