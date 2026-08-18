use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::ProcessInfo;
use core::fmt;
use std::fmt::Formatter;

/// Represents the statistics of disk activity for a specific process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct DiskUsage {
    /// Number of bytes read from disk since the last update.
    pub read_bytes: u64,
    /// Number of bytes written to disk since the last update.
    pub written_bytes: u64,
    /// Total number of bytes read from disk since the process started.
    pub total_read_bytes: u64,
    /// Total number of bytes written to disk since the process started.
    pub total_written_bytes: u64,
}

/// Represents the current execution state of a process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// The state of the process cannot be determined.
    #[default]
    Unknown,
    /// The process is actively running on a CPU core.
    Run,
    /// The process is idle and waiting for work (common on BSD).
    Idle,
    /// The process finished execution but its parent has not read its exit code yet.
    Zombie,
    /// The process is sleeping and waiting for an event or signal.
    Sleep,
    /// The process is being inspected by a debugger or tracing tool.
    Tracing,
    /// The process was stopped by a signal (like SIGSTOP).
    Stop,
    /// The process is dead and completely terminated.
    Dead,
    /// The process is in a deep sleep but will wake up to handle a fatal signal.
    Wakekill,
    /// The process is currently moving from sleep to a running state.
    Waking,
    /// The process thread is parked (usually for internal kernel management).
    Parked,
    /// The process is blocked waiting for a lock.
    LockBlocked,
    /// The process is in a deep sleep waiting for disk Input/Output operations.
    UninterruptibleDiskSleep,
    /// The process is suspended or paused (common on macOS).
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
