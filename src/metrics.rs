use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface_name: String,

    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub total_rx_package: u64,
    pub total_tx_package: u64,
    pub total_rx_errors: u64,
    pub total_tx_errors: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CpuInfo {
    pub cpu_usage: f32,
    pub load_avg: LoadAverage,
    pub threads: usize,
    pub physical_core_count: usize,
    pub components_info: Vec<ComponentInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub temp: f64,
}

#[derive(Serialize, Deserialize)]
pub struct LoadAverage {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}
