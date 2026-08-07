use crate::metrics::additional_structs::*;
use crate::metrics::{LoadAverage, ProcessInfo, ProcessesInfo};
use sysinfo::{CpuRefreshKind, LoadAvg, Process, ProcessStatus as SysProcessStatus, System};

impl From<sysinfo::DiskUsage> for DiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        DiskUsage {
            read_bytes: value.read_bytes,
            written_bytes: value.written_bytes,
            total_read_bytes: value.total_read_bytes,
            total_written_bytes: value.total_written_bytes,
        }
    }
}

impl From<SysProcessStatus> for ProcessStatus {
    fn from(value: SysProcessStatus) -> Self {
        match value {
            SysProcessStatus::Idle => ProcessStatus::Idle,
            SysProcessStatus::Run => ProcessStatus::Run,
            SysProcessStatus::Sleep => ProcessStatus::Sleep,
            SysProcessStatus::Stop => ProcessStatus::Stop,
            SysProcessStatus::Zombie => ProcessStatus::Zombie,
            SysProcessStatus::Tracing => ProcessStatus::Tracing,
            SysProcessStatus::Dead => ProcessStatus::Dead,
            SysProcessStatus::Wakekill => ProcessStatus::Wakekill,
            SysProcessStatus::Waking => ProcessStatus::Waking,
            SysProcessStatus::Parked => ProcessStatus::Parked,
            SysProcessStatus::LockBlocked => ProcessStatus::LockBlocked,
            SysProcessStatus::UninterruptibleDiskSleep => ProcessStatus::UninterruptibleDiskSleep,
            SysProcessStatus::Suspended => ProcessStatus::Suspended,
            _ => ProcessStatus::Unknown,
        }
    }
}

impl From<&Process> for ProcessInfo {
    fn from(value: &Process) -> Self {
        let mut cpu_cores = System::new();
        cpu_cores.refresh_cpu_list(CpuRefreshKind::everything().without_cpu_usage());
        let cpu_cores = cpu_cores.cpus().len() as f32;
        let cpu_usage = value.cpu_usage();
        let calculate_cpu_usage = if cpu_cores != 0.0 && cpu_usage != 0.0 {
            cpu_usage / cpu_cores
        } else {
            0.0
        };

        let uid = value
            .user_id()
            .map(|id| id.to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unknown".to_string());
        let gid = value
            .group_id()
            .map(|id| id.to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unknown".to_string());
        ProcessInfo {
            name: value.name().to_str().unwrap().to_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: DiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_string(),
            cpu_usage: calculate_cpu_usage,
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
        }
    }
}

impl From<LoadAvg> for LoadAverage {
    fn from(value: LoadAvg) -> Self {
        LoadAverage {
            one: value.one,
            five: value.five,
            fifteen: value.fifteen,
        }
    }
}
