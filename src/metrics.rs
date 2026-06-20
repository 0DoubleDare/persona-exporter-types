use std::path::Components;
use serde::{Deserialize, Serialize};

/// General structure that contains all the metrics of the working machine
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ServerMetrics {
    pub system: Option<SystemInfo>,
    pub disk: Option<DiskInfo>,
    pub network: Option<NetworkInfo>,
    pub cpu: Option<CpuInfo>,
    pub components: Option<ComponentsInfo>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SystemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,

    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,

    /// Metric showing the average load on processor threads
    pub load_avg: LoadAverage,
}

///
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub file_system: String,
    pub kind: String,
    pub total_space: u64,
    pub available_space: u64,
}

/// Network information
#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentsInfo {
    pub count: usize,
    pub is_empty: bool,
    pub components: Vec<ComponentInfo>
}
/// Statistics on the machine's processor
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CpuInfo {
    /// System CPU usage. Measured as a percentage from 0%-100%
    pub cpu_usage: f32,
    /// Number of processor threads
    pub threads: usize,
    /// Number of physical processor cores
    pub physical_core_count: usize,
}

/// Processor thread information
#[derive(Serialize, Deserialize, Default, Debug)]
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
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoadAverage {
    /// LA at one minute
    pub one: f64,
    /// LA at five minutes
    pub five: f64,
    /// LA at fifteen minutes
    pub fifteen: f64,
}
