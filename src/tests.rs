use crate::buffer::StackBuffer;
use crate::io;
use crate::sink::MemorySink;
use crate::time_date::DTime;
use crate::FileStreamer;
use crate::LogAnalyzer;
use crate::LogEntry;
use crate::LogLevel;
use core::fmt::Write;
use std::fs::File;

#[test]
fn test_macro_output_formatting() {
    let mut sink = MemorySink::<512>::new();

    info!(
        &mut sink =>
        "AUTH", "User {} connected from {}", 101, "127.0.0.1"
    );

    let output = sink.as_str();
    assert!(output.contains("[INFO]"));
    assert!(output.contains("[AUTH]"));
    assert!(output.contains("User 101 connected from 127.0.0.1"));
}

#[test]
fn test_stack_buffer_overflow_safety() {
    let mut buffer = StackBuffer::<10>::new();

    let result = write!(buffer, "This text is way too long for 10 bytes");

    assert!(result.is_err());
    assert_eq!(buffer.as_bytes().len(), 10);
    assert_eq!(buffer.as_str(), "This text ");
}

#[test]
fn test_dtime_known_timestamps() {
    let dt_epoch = DTime::from_unix(0);
    assert_eq!(format!("{}", dt_epoch), "1970-01-01 00:00:00");

    let dt_custom = DTime::from_unix(1700000000);
    assert_eq!(format!("{}", dt_custom), "2023-11-14 22:13:20");
}

#[test]
fn test_invalid_log_parsing() {
    assert!(LogEntry::parse("").is_none());
    assert!(LogEntry::parse("Invalid log without brackets").is_none());
    assert!(LogEntry::parse("[100] [INFO] Unclosed target").is_none());
}
use crate::sink::StdoutSink;

#[test]
fn test_zero_alloc_macros() {
    let mut sink = StdoutSink;
    let user_id = 42;
    let ip = "192.168.1.50";

    info!(&mut sink => "auth", "User {} logged in from {}", user_id, ip);
}

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
fn test_no_pass_sink() {
    let vr = "0.2.3";
    info!("main", "Application started with version {}", vr);
}
