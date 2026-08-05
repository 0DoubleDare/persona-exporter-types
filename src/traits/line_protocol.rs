use crate::metrics::*;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

impl From<SystemInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: SystemInfo) -> Self {
        let mut distro_like = value.distribution_id_like.join(",");
        if distro_like.is_empty() {
            distro_like = "None".to_string()
        }

        let mut line_builder = LineProtocolBuilder::new()
            .measurement("system")
            .tag("name", &value.name)
            .tag("kernel_version", &value.kernel_version)
            .tag("kernel_long_version", &value.kernel_long_version)
            .tag("distribution_id", &value.distribution_id)
            .tag("distribution_id_like", distro_like.as_str())
            .tag("cpu_arch", &value.cpu_arch)
            .tag("os_version", &value.os_version)
            .tag("host_name", &value.host_name)
            .field("boot_time", value.boot_time)
            .field("uptime", value.uptime)
            .field("load_avg_one", value.load_average.one as f64)
            .field("load_avg_five", value.load_average.five as f64)
            .field("load_avg_fifteen", value.load_average.fifteen as f64)


    }
}

impl From<MemoryInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: MemoryInfo) -> Self {
        LineProtocolBuilder::new()
            .measurement("memory")
            .field("total_memory", value.total_memory)
            .field("used_memory", value.used_memory)
            .field("free_memory", value.free_memory)
            .field("available_memory", value.available_memory)
            .field("total_swap", value.total_swap)
            .field("used_swap", value.used_swap)
            .field("free_swap", value.free_swap)
    }
}

impl From<DiskInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: DiskInfo) -> Self {
        LineProtocolBuilder::new()
            .measurement("disk")
            .tag("name", &value.name)
            .tag("file_system", &value.kind)
            .tag("kind", &value.kind)
            .field("total_space", value.total_space)
            .field("available_space", value.available_space)
    }
}

impl From<NetworkInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: NetworkInfo) -> Self {
        LineProtocolBuilder::new()
            .measurement("network")
            .tag("interface_name", &value.interface_name)
            .field("total_rx_bytes", value.total_rx_bytes)
            .field("total_rx_packets", value.total_tx_packets)
            .field("total_rx_errors", value.total_rx_errors)
            .field("total_tx_bytes", value.total_tx_bytes)
            .field("total_tx_packets", value.total_tx_packets)
            .field("total_tx_errors", value.total_tx_errors)
    }
}

impl From<CpuInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: CpuInfo) -> Self {
        LineProtocolBuilder::new()
            .measurement("cpu")
            .field("usage", value.cpu_usage as f64)
            .field("threads", value.threads as f64)
            .field("physical_core_count", value.physical_core_count as i64)
    }
}

impl From<ComponentInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from(value: ComponentInfo) -> Self {
        LineProtocolBuilder::new()
            .measurement("component")
            .tag("id", &value.id)
            .tag("name", &value.name)
            .field("temp", value.temp as i64)
            .field("critical_temp", value.critical_temp as i64)
            .field("max_temp", value.max_temp as i64)
    }
}
