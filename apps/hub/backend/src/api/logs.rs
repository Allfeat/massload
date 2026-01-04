//! Real-time log streaming via Server-Sent Events (SSE).
//!
//! This module provides a broadcast channel for pipeline logs
//! that can be streamed to frontend clients via SSE.
//!
//! Logs are also persisted to disk for later consultation.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
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
    /// Timestamp (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl LogEntry {
    pub fn info(message: impl Into<String>) -> Self {
        Self { 
            level: LogLevel::Info, 
            message: message.into(), 
            indent: 0,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    
    pub fn success(message: impl Into<String>) -> Self {
        Self { 
            level: LogLevel::Success, 
            message: message.into(), 
            indent: 0,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    
    pub fn warning(message: impl Into<String>) -> Self {
        Self { 
            level: LogLevel::Warning, 
            message: message.into(), 
            indent: 0,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self { 
            level: LogLevel::Error, 
            message: message.into(), 
            indent: 0,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
    
    pub fn with_indent(mut self, indent: u8) -> Self {
        self.indent = indent;
        self
    }
}

/// Log session for tracking a single upload/processing workflow
#[derive(Debug, Clone)]
pub struct LogSession {
    pub id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub log_file: PathBuf,
}

impl LogSession {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4().to_string();
        
        // Create logs directory if it doesn't exist
        let logs_dir = PathBuf::from("logs");
        if let Err(e) = fs::create_dir_all(&logs_dir) {
            eprintln!("Failed to create logs directory: {}", e);
        }
        
        // Create log file: logs/YYYY-MM-DD/session-{uuid}.log
        let date_dir = logs_dir.join(now.format("%Y-%m-%d").to_string());
        if let Err(e) = fs::create_dir_all(&date_dir) {
            eprintln!("Failed to create date directory: {}", e);
        }
        
        let log_file = date_dir.join(format!("session-{}.log", &id[..8]));
        
        Self {
            id,
            started_at: now,
            log_file,
        }
    }
    
    /// Write a log entry to the session file
    pub fn write_log(&self, entry: &LogEntry) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            let timestamp = entry.timestamp.as_deref().unwrap_or("N/A");
            let level = format!("{:?}", entry.level).to_uppercase();
            let indent = "  ".repeat(entry.indent as usize);
            let line = format!("[{}] {:7} {}{}\n", timestamp, level, indent, entry.message);
            
            if let Err(e) = file.write_all(line.as_bytes()) {
                eprintln!("Failed to write log to file: {}", e);
            }
        }
    }
}

/// Global log broadcaster
pub static LOG_BROADCASTER: Lazy<LogBroadcaster> = Lazy::new(LogBroadcaster::new);

/// Broadcasts log entries to all connected SSE clients
pub struct LogBroadcaster {
    sender: broadcast::Sender<LogEntry>,
    current_session: Arc<Mutex<Option<LogSession>>>,
}

impl LogBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { 
            sender,
            current_session: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Start a new log session
    pub fn start_session(&self) -> String {
        let session = LogSession::new();
        let session_id = session.id.clone();
        
        // Write session header
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&session.log_file)
        {
            let header = format!(
                "================================================================================\n\
                 Session ID: {}\n\
                 Started at: {}\n\
                 ================================================================================\n\n",
                session.id,
                session.started_at.to_rfc3339()
            );
            let _ = file.write_all(header.as_bytes());
        }
        
        println!("\n📝 Log session started: {}", session_id);
        println!("   Log file: {}", session.log_file.display());
        
        if let Ok(mut current) = self.current_session.lock() {
            *current = Some(session);
        }
        
        session_id
    }
    
    /// End the current log session
    pub fn end_session(&self) {
        if let Ok(mut current) = self.current_session.lock() {
            if let Some(session) = current.take() {
                // Write session footer
                if let Ok(mut file) = OpenOptions::new()
                    .append(true)
                    .open(&session.log_file)
                {
                    let ended_at = chrono::Utc::now();
                    let duration = ended_at.signed_duration_since(session.started_at);
                    let footer = format!(
                        "\n================================================================================\n\
                         Session ended at: {}\n\
                         Duration: {:.2}s\n\
                         ================================================================================\n",
                        ended_at.to_rfc3339(),
                        duration.num_milliseconds() as f64 / 1000.0
                    );
                    let _ = file.write_all(footer.as_bytes());
                }
                
                println!("📝 Log session ended: {}\n", session.id);
            }
        }
    }
    
    /// Send a log entry to all subscribers
    pub fn log(&self, entry: LogEntry) {
        // Print to stdout
        let prefix = match entry.level {
            LogLevel::Info => "   ",
            LogLevel::Success => "   ✓",
            LogLevel::Warning => "   ⚠️",
            LogLevel::Error => "   ❌",
        };
        let indent = "   ".repeat(entry.indent as usize);
        println!("{}{} {}", indent, prefix, entry.message);
        
        // Write to log file if session is active
        if let Ok(current) = self.current_session.lock() {
            if let Some(session) = current.as_ref() {
                session.write_log(&entry);
            }
        }
        
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

