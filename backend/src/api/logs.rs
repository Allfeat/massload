//! Real-time log streaming via Server-Sent Events (SSE).
//!
//! This module provides a broadcast channel for pipeline logs
//! that can be streamed to frontend clients via SSE.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Log level for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// A single log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Optional indentation level (for nested logs)
    #[serde(default)]
    pub indent: u8,
}

impl LogEntry {
    pub fn info(message: impl Into<String>) -> Self {
        Self { level: LogLevel::Info, message: message.into(), indent: 0 }
    }
    
    pub fn success(message: impl Into<String>) -> Self {
        Self { level: LogLevel::Success, message: message.into(), indent: 0 }
    }
    
    pub fn warning(message: impl Into<String>) -> Self {
        Self { level: LogLevel::Warning, message: message.into(), indent: 0 }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self { level: LogLevel::Error, message: message.into(), indent: 0 }
    }
    
    pub fn with_indent(mut self, indent: u8) -> Self {
        self.indent = indent;
        self
    }
}

/// Global log broadcaster
pub static LOG_BROADCASTER: Lazy<LogBroadcaster> = Lazy::new(LogBroadcaster::new);

/// Broadcasts log entries to all connected SSE clients
pub struct LogBroadcaster {
    sender: broadcast::Sender<LogEntry>,
}

impl LogBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }
    
    /// Send a log entry to all subscribers
    pub fn log(&self, entry: LogEntry) {
        // Also print to stdout
        let prefix = match entry.level {
            LogLevel::Info => "   ",
            LogLevel::Success => "   ✓",
            LogLevel::Warning => "   ⚠️",
            LogLevel::Error => "   ❌",
        };
        let indent = "   ".repeat(entry.indent as usize);
        println!("{}{} {}", indent, prefix, entry.message);
        
        // Broadcast to SSE clients (ignore if no receivers)
        let _ = self.sender.send(entry);
    }
    
    /// Get a receiver for SSE streaming
    pub fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.sender.subscribe()
    }
}

impl Default for LogBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenient logging functions
pub fn log_info(msg: impl Into<String>) {
    LOG_BROADCASTER.log(LogEntry::info(msg));
}

pub fn log_success(msg: impl Into<String>) {
    LOG_BROADCASTER.log(LogEntry::success(msg));
}

pub fn log_warning(msg: impl Into<String>) {
    LOG_BROADCASTER.log(LogEntry::warning(msg));
}

pub fn log_error(msg: impl Into<String>) {
    LOG_BROADCASTER.log(LogEntry::error(msg));
}

pub fn log_info_indent(msg: impl Into<String>, indent: u8) {
    LOG_BROADCASTER.log(LogEntry::info(msg).with_indent(indent));
}

pub fn log_success_indent(msg: impl Into<String>, indent: u8) {
    LOG_BROADCASTER.log(LogEntry::success(msg).with_indent(indent));
}

