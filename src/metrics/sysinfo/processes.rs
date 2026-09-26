use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::structs::common::DiskUsage;
use crate::metrics::structs::processes::{ProcessInfo, ProcessStatus};
use compact_str::ToCompactString;
use sysinfo::Process;
use sysinfo::ProcessStatus as SysProcessStatus;

pub trait FromWithNormalizeCpu<T> {
    fn from_with_cpu(value: T, cpu_thread_count: f32) -> Self;
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

// impl ProcessInfo {
//     pub fn from_process(value: &mut Process) -> Self {
//         let raw_cpu_usage = value.cpu_usage();
//
//         let calculate_cpu_usage = if raw_cpu_usage != 0.0 || cpu_cores != 0 {
//           raw_cpu_usage / cpu_cores as f32
//         } else { 0.0 };
//
// let uid = value
//     .user_id()
//     .map(|id| id.to_compact_string())
//     .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
// let gid = value
//     .group_id()
//     .map(|id| id.to_compact_string())
//     .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
// ProcessInfo {
//     name: value.name().to_str().unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE).to_compact_string(),
//     status: ProcessStatus::from(value.status()),
//     disk_usage: ProcessDiskUsage::from(value.disk_usage()),
//     program_id: value.pid().to_compact_string(),
//     cpu_usage: value.cpu_usage(),
//     memory_usage: value.memory(),
//     virtual_memory: value.virtual_memory(),
//     run_time: value.run_time(),
//     start_time: value.start_time(),
//     user_id: uid,
//     group_id: gid,
// }
// }
// }

impl FromWithNormalizeCpu<&Process> for ProcessInfo {
    fn from_with_cpu(value: &Process, cpu_thread_count: f32) -> Self {
        let uid = value
            .user_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
        let gid = value
            .group_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
        // .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());

        ProcessInfo {
            name: value
                .name()
                .to_str()
                .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE)
                .to_compact_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: DiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_compact_string(),
            cpu_usage_per_thread: value.cpu_usage(),
            global_cpu_usage: value.cpu_usage() / cpu_thread_count,
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
        }
    }
}

impl From<&Process> for ProcessInfo {
    fn from(value: &Process) -> Self {
        let uid = value
            .user_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
        let gid = value
            .group_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());

        ProcessInfo {
            name: value
                .name()
                .to_str()
                .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE)
                .to_compact_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: DiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_compact_string(),
            cpu_usage_per_thread: 0.0,
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
            global_cpu_usage: 0.0,
        }
    }
}
