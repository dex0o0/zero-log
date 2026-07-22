#[macro_use]
pub mod macros;

use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

/// ==========================
/// 1.Log Levels
/// ==========================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

/// ===============================
/// 2. Logger (Writung Logs)
/// ===============================
pub struct Logger {
    file: Option<File>,
}

impl Logger {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file: Some(file) })
    }

    pub fn stdout() -> Self {
        Self { file: None }
    }

    pub fn log(&mut self, level: LogLevel, target: &str, message: &str) -> io::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let formatted = format!(
            "[{}] [{}] [{}] {}\n",
            timestamp,
            level.as_str(),
            target,
            message
        );

        if let Some(ref mut f) = self.file {
            f.write_all(formatted.as_bytes())?;
        } else {
            print!("{}", formatted);
        }
        Ok(())
    }

    pub fn info(&mut self, target: &str, message: &str) -> io::Result<()> {
        self.log(LogLevel::Info, target, message)
    }
    pub fn error(&mut self, target: &str, message: &str) -> io::Result<()> {
        self.log(LogLevel::Error, target, message)
    }
}

pub struct LogEntry<'a> {
    pub timestamp: &'a str,
    pub level: &'a str,
    pub target: &'a str,
    pub message: &'a str,
}

impl<'a> LogEntry<'a> {
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

pub struct LogAnalyzer<'a> {
    raw_data: &'a str,
}

impl<'a> LogAnalyzer<'a> {
    pub fn new(raw_data: &'a str) -> Self {
        Self { raw_data }
    }

    pub fn entries(&self) -> impl Iterator<Item = LogEntry<'a>> {
        self.raw_data.lines().filter_map(LogEntry::parse)
    }

    pub fn filter_by_level(&self, level: LogLevel) -> impl Iterator<Item = LogEntry<'a>> {
        let target_level = level.as_str();
        self.entries()
            .filter(move |entry| entry.level == target_level)
    }
}

/// =======================
/// Log starts_with
/// =======================

#[derive(Debug, Clone, Copy, Default)]
pub struct LogStats {
    pub total_entries: usize,
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
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

pub struct FileStreamer;

impl FileStreamer {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_zero_copy_parsing() {
        let raw_log = "[1721580000] [ERROR] [auth_service] Invalid password attempt";
        let entry = LogEntry::parse(raw_log).expect("Failed to parse");

        assert_eq!(entry.timestamp, "1721580000");
        assert_eq!(entry.level, "ERROR");
        assert_eq!(entry.target, "auth_service");
        assert_eq!(entry.message, "Invalid password attempt");
    }

    #[test]
    fn test_zero_copy_analyzer_filtering() {
        let logs = "\
[1721580000] [INFO] [server] Server started
[1721580001] [ERROR] [db] Connection lost
[1721580002] [INFO] [server] Client connected
";
        let analyzer = LogAnalyzer::new(logs);
        let errors: Vec<_> = analyzer.filter_by_level(LogLevel::Error).collect();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target, "db");
        assert_eq!(errors[0].message, "Connection lost");
    }

    #[test]
    fn test_file_streamer_and_stats() -> io::Result<()> {
        use std::io::Write;

        // cteate file for test
        let test_path = "test_stream.log";
        {
            let mut file = File::create(test_path)?;
            writeln!(file, "[100] [INFO] [sys] Booting")?;
            writeln!(file, "[101] [ERROR] [net] Disconnected")?;
            writeln!(file, "[102] [ERROR] [db] Query timeout")?;
        }

        // ۱. test analyzer on file
        let stats = FileStreamer::analyze_file(test_path)?;
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.error_count, 2);
        assert_eq!(stats.info_count, 1);

        // ۲. fillter test in streaming
        let mut errors_found = 0;
        FileStreamer::stream(test_path, |entry| {
            if entry.level == "ERROR" {
                errors_found += 1;
            }
        })?;
        assert_eq!(errors_found, 2);

        // clear test file
        let _ = std::fs::remove_file(test_path);
        Ok(())
    }

    #[test]
    fn test_log_macros() -> io::Result<()> {
        let test_path = "test_macro.log";
        {
            let mut logger = Logger::new(test_path)?;
            let user_id = 42;
            let ip = "192.168.1.50";

            info!(logger, "auth", "User {} logged in successfuly", user_id);
            error!(logger, "net", "Connection failed from IP: {}", ip);
        }

        let stats = FileStreamer::analyze_file(test_path)?;

        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.info_count, 1);
        assert_eq!(stats.error_count, 1);

        let _ = fs::remove_file(test_path);
        Ok(())
    }
}
