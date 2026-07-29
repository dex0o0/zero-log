//! # zero-log
//!
//! `zero-log` is an ultra-fast, zero-dependency, and zero-allocation log parser
//! and streaming analysis library for Rust.
//!
//! ## key Features
//! - **Zero Dependencies:** Compile in milliseconds with no external crates.
//! - **Zero Allocation Parsing:** Utilizes string slices (`&str`) to avoid heap allocations.
//! - **Streaming Processing:** Efficiently analyzes massive log files line-by-line using a single buffer.
//!
//! ## Quick Start
//! ```rust
//! use zero_log::LogEntry;
//!
//! let line = "[1721580000] [ERROR] [auth] Failed login attempt";
//! if let Some(entry) = LogEntry::parse(line){
//!     assert_eq!(entry.level, "ERROR" );
//!     assert_eq!(entry.target, "auth" );
//! }
//!
//! ```

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

pub mod buffer;
pub mod event;
pub mod sink;
pub mod time_date;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Represents the severity level of a log entry.
///
/// # Examples
/// ```rust
/// use zero_log::LogLevel;
///
/// let level = LogLevel::Error;
/// assert_eq!(level.as_str(), "ERROR" );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Represents critical errors or failure events.
    Error,
    /// Represents warning conditions that might require attention.
    Warn,
    /// Represents general informational messages.
    Info,
    /// Represents detailed diagnostic information for debuging.
    Debug,
}

impl LogLevel {
    /// Converts the [`LogLevel`] variant into its uppercase string slice representation (`&'static str`).
    ///
    /// # Examples
    /// ```rust
    /// use zero_log::LogLevel;
    ///
    /// assert_eq!(LogLevel::Info.as_str(), "INFO" );
    /// assert_eq!(LogLevel::Warn.as_str(), "WARN" );
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

/// Represents a parsed log entry holding zero-copy refrences to the original log line.
///
/// All fields are string slice (`&str`) pointing directly to the underlying string buffer,
/// ensuring high perfomance without memory allocations.
pub struct LogEntry<'a> {
    /// Unix timestamp or formatted date string of the log event.
    pub timestamp: &'a str,
    /// Severity level of the log (e.g., `INFO`,`WARN`,`ERROR`).
    pub level: &'a str,
    /// Component or target module that generated the log.
    pub target: &'a str,
    /// The actual log message content.
    pub message: &'a str,
}

impl<'a> LogEntry<'a> {
    /// Parses a raw log line slice into a [`LogEntry`]
    ///
    /// Expected format: `[timestamp] [level] [target] message`
    ///
    /// # Returns
    /// - `Some(LogEntry)` if the line matches the expected structure.
    /// - `None` if the line is malformed or empty.
    ///
    /// # Examples
    /// ```rust
    /// use zero_log::LogEntry;
    ///
    /// let raw_line = "[1721580000] [WARN] [database] High latency detected";
    /// let entry = LogEntry::parse(raw_line).unwrap();
    ///
    /// assert_eq!(entry.timestamp, "1721580000" );
    /// assert_eq!(entry.level, "WARN" );
    /// assert_eq!(entry.target, "database" );
    /// assert_eq!(entry.message, "High latency detected");
    /// ```
    pub fn parse(line: &'a str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let (timestamp, rest) = extract_bracket(line)?;
        let (level, rest) = extract_bracket(rest.trim_start())?;
        let (target, rest) = extract_bracket(rest.trim_start())?;
        let message = rest.trim_start();

        Some(Self {
            timestamp,
            level,
            target,
            message,
        })
    }
}

fn extract_bracket(input: &str) -> Option<(&str, &str)> {
    if !input.starts_with('[') {
        return None;
    }
    let end_idx = input.find(']')?;
    let content = &input[1..end_idx];
    let remaining = &input[end_idx + 1..];
    Some((content, remaining))
}

/// An in-memory, zero-copy log analyzer that operates on raw log string slices (`&str`).
///
/// Provides iterator-based operations to filter and inspect log entries without memory allocation.
///
/// # Examples
/// ```rust
/// use zero_log::{LogAnalyzer,LogLevel};
///
/// let raw_data = "\
/// [100] [INFO] [sys] Ready
/// [101] [ERROR] [auth] Failed login
/// ";
///
///let analyzer = LogAnalyzer::new(raw_data);
///let errors:Vec<_> = analyzer.filter_by_level(LogLevel::Error).collect();
///
///assert_eq!(errors.len(), 1 );
///assert_eq!(errors[0].target , "auth" );
/// ```
pub struct LogAnalyzer<'a> {
    raw_data: &'a str,
}

impl<'a> LogAnalyzer<'a> {
    /// Creates a new [`LogAnalyzer`] wrapped around a raw log string slice.
    pub fn new(raw_data: &'a str) -> Self {
        Self { raw_data }
    }

    /// Returns an iterator yielding parsed [`LogEntry`] items from the raw log slice.
    ///
    /// malformed lines that fail to patse are automatically skipped.
    pub fn entries(&self) -> impl Iterator<Item = LogEntry<'a>> {
        self.raw_data.lines().filter_map(LogEntry::parse)
    }

    /// Returns an iterator that filtering log entries matching the specified [`LogLevel`].
    pub fn filter_by_level(&self, level: LogLevel) -> impl Iterator<Item = LogEntry<'a>> {
        let target_level = level.as_str();
        self.entries()
            .filter(move |entry| entry.level == target_level)
    }
}

/// Statistics summary generated by analyzing log files.
#[derive(Debug, Clone, Copy, Default)]
pub struct LogStats {
    /// Total number of processed lines.
    pub total_entries: usize,
    /// Total count of `ERROR` level logs.
    pub error_count: usize,
    /// Total count of `WARN` level logs,
    pub warn_count: usize,
    /// Total count of `INFO` level logs,
    pub info_count: usize,
    /// Total count of `Debug` level logs,
    pub debug_count: usize,
}

impl LogStats {
    pub fn process(&mut self, entry: &LogEntry) {
        self.total_entries += 1;
        match entry.level {
            "ERROR" => self.error_count += 1,
            "WARN" => self.warn_count += 1,
            "INFO" => self.info_count += 1,
            "DEBUG" => self.debug_count += 1,
            _ => {}
        }
    }
}

/// High-performance log file streamer and analyzer.
pub struct FileStreamer;

impl FileStreamer {
    /// Streams a log file line-by-line and calcutes aggregated statistics.
    ///
    /// Uses a single reusable buffer to maintain zero heap re-allocation during execution.
    ///
    /// # Errors
    /// Returns an [`std::io::Result::Err`] if the file cannot be opened or read.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use zero_log::FileStreamer;
    ///
    /// let stats = FileStreamer::analyze_file("server.log").unwrap();
    /// println!("Total lines processed: {}",stats.total_entries);
    /// println!("Errors found: {}",stats.error_count);
    /// ```
    pub fn stream<P, F>(path: P, mut callbak: F) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&LogEntry),
    {
        let file = File::open(path)?;
        let mut reader = io::BufReader::new(file);
        let mut line_buf = String::with_capacity(512);

        while reader.read_line(&mut line_buf)? > 0 {
            if let Some(entry) = LogEntry::parse(&line_buf) {
                callbak(&entry);
            }
            line_buf.clear();
        }

        Ok(())
    }

    pub fn analyze_file<P: AsRef<Path>>(path: P) -> io::Result<LogStats> {
        let mut stats = LogStats::default();
        Self::stream(path, |entry| {
            stats.process(entry);
        })?;

        Ok(stats)
    }
}
