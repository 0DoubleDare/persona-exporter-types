use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::ProcessInfo;
use core::fmt;
use std::fmt::Formatter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DiskUsage {
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub total_read_bytes: u64,
    pub total_written_bytes: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    #[default]
    Unknown,
    Run,
    Idle,
    Zombie,
    Sleep,
    Tracing,
    Stop,
    Dead,
    Wakekill,
    Waking,
    Parked,
    LockBlocked,
    UninterruptibleDiskSleep,
    Suspended,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for ProcessInfo {
    fn default() -> Self {
        ProcessInfo {
            name: DEFAULT_UNKNOWN_MESSAGE.to_string(),
            status: ProcessStatus::default(),
            disk_usage: DiskUsage::default(),
            program_id: DEFAULT_UNKNOWN_MESSAGE.to_string(),
            cpu_usage: 0.0,
            memory_usage: 0,
            virtual_memory: 0,
            run_time: 0,
            start_time: 0,
            user_id: DEFAULT_UNKNOWN_MESSAGE.to_string(),
            group_id: DEFAULT_UNKNOWN_MESSAGE.to_string(),
        }
    }
}
