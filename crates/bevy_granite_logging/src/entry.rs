use crate::config::{LogCategory, LogLevel, LogType};

// used for buffer

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub log_type: LogType,
    pub level: LogLevel,
    pub category: LogCategory,
    pub message: String,
}

