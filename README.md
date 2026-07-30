# zero-log

An ultra-fast, zero-dependency, and zero-allocation log streaming and analysis library for Rust.

zero-log is designed for performance: zero external crates, zero heap allocations during parsing and streaming, and a small, ergonomic API for both emitting and analyzing logs.

## Key features

- Zero dependencies — compiles quickly with no external crates.
- Zero-allocation parsing — uses string slices (&str) for parsed fields.
- Streaming processing — single reusable buffer for line-by-line processing of very large log files.
- Pluggable sinks — `LogSink` trait with `StdoutSink`, `FileSink` and test `MemorySink`.
- Lightweight logging macros — `info!`, `error!`, `warn!`, `debug!` that accept an optional sink to remain zero-allocation.
- Human-readable UTC timestamps — `DTime` provides formatted date/time strings in `YYYY-MM-DD HH:MM:SS` (UTC).
- Fast analysis utilities — `LogAnalyzer`, `FileStreamer`, and `LogStats`.

## Installation

From crates.io (when published):

```toml
[dependencies]
zero-log = "0.3.1"
```

or for local development:

```toml
[dependencies]
zero-log = { path = "../zero-log" }
```

## Quick start

This crate exposes zero-allocation macros and small helpers for parsing and streaming logs.

### Emit logs with macros (default: stdout)

Macros default to `StdoutSink` when no sink is supplied:

```rust
use zero_log::{info, error};

fn main() {
    info!("server", "Server started");
    error!("db", "Connection failed: code {}", 42);
}
```

### Emit logs with a custom sink (zero-allocation)

Pass a mutable sink to the macro to avoid allocating and to control where logs go:

```rust
use zero_log::sink::FileSink;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("app.log")?;
    let mut sink = FileSink::new(file);

    info!(&mut sink => "auth", "User {} logged in", 1001);
    error!(&mut sink => "net", "Timeout from {}", "10.0.0.1");

    Ok(())
}
```

The macro forms:

- info!(target, "fmt", ...)
- info!(&mut sink => target, "fmt", ...)
- same for `error!`, `warn!`, `debug!`

### Parse a single log line (zero-copy)

```rust
use zero_log::LogEntry;

fn main() {
    let line = "[2026-07-29 14:32:01] [ERROR] [auth] Invalid password attempt";
    if let Some(entry) = LogEntry::parse(line) {
        assert_eq!(entry.level, "ERROR");
        assert_eq!(entry.target, "auth");
        println!("{}: {}", entry.timestamp, entry.message);
    }
}
```

### Analyze logs in-memory

`LogAnalyzer` operates on a `&str` and yields zero-copy `LogEntry` items:

```rust
use zero_log::{LogAnalyzer, LogLevel};

fn main() {
    let data = "\
[2026-07-29 14:32:01] [INFO] [server] Started
[2026-07-29 14:32:02] [ERROR] [db] Connection lost
";

    let analyzer = LogAnalyzer::new(data);
    for err in analyzer.filter_by_level(LogLevel::Error) {
        println!("[{}] {}: {}", err.timestamp, err.target, err.message);
    }
}
```

### Stream large files with a reusable buffer (zero allocations)

`FileStreamer::stream` iterates lines with a single reusable buffer and calls your closure for each parsed `LogEntry`:

```rust
use zero_log::FileStreamer;

fn main() -> std::io::Result<()> {
    FileStreamer::stream("server.log", |entry| {
        if entry.level == "ERROR" {
            println!("Error in {}: {}", entry.target, entry.message);
        }
    })?;
    Ok(())
}
```

You can also get aggregated statistics:

```rust
use zero_log::FileStreamer;

fn main() -> std::io::Result<()> {
    let stats = FileStreamer::analyze_file("server.log")?;
    println!("Total logs: {}", stats.total_entries);
    println!("Errors: {}", stats.error_count);
    println!("Warnings: {}", stats.warn_count);
    println!("Info: {}", stats.info_count);
    println!("Debug: {}", stats.debug_count);
    Ok(())
}
```

## Log format

Expected log line format (used by parser and analyzer):

```
[timestamp] [LEVEL] [target] message
```

Example:

```
[2026-07-29 14:32:01] [ERROR] [auth_service] Invalid password attempt
[2026-07-29 14:32:01] [INFO] [server] Server started successfully
[2026-07-29 14:32:02] [WARN] [db] Connection pool nearly exhausted
```

Timestamps are emitted by the built-in `DTime` helper and formatted as `YYYY-MM-DD HH:MM:SS` in UTC (e.g. `[2026-07-29 14:32:01]`).

## API overview

- LogEntry
  - `LogEntry::parse(&str) -> Option<LogEntry>` — zero-copy parser for a single line.

- LogLevel
  - Enum: `Error`, `Warn`, `Info`, `Debug`
  - `as_str()` returns `"ERROR" | "WARN" | "INFO" | "DEBUG"`

- LogAnalyzer
  - `LogAnalyzer::new(&str)` — build analyzer from log content.
  - `entries()` — iterator over parsed entries.
  - `filter_by_level(LogLevel)` — iterator filtered by level.

- FileStreamer
  - `FileStreamer::stream(path, callback)` — stream file lines with a reusable buffer.
  - `FileStreamer::analyze_file(path)` — return `LogStats`.

- LogStats
  - `total_entries`, `error_count`, `warn_count`, `info_count`, `debug_count`

- LogSink trait and sinks (`sink` module)
  - `LogSink` trait: `write_log(&mut self, data: &[u8])`
  - Provided sinks:
    - `StdoutSink` — writes to stdout
    - `FileSink` — writes to a `std::fs::File`
    - `MemorySink<N>` — test-only in-memory sink used by unit tests

- Macros
  - `info!(...)`, `error!(...)`, `warn!(...)`, `debug!(...)`
  - Forms:
    - `info!(target, "fmt", ...)` — uses stdout sink
    - `info!(&mut sink => target, "fmt", ...)` — uses provided sink

## Internals & design notes

- StackBuffer: small stack-allocated buffer used by logging macros to avoid heap allocations.
- `event::log_event` writes formatted lines into the stack buffer and then calls the sink.
- `time_date::DTime` provides conversion from unix seconds to human-readable date/time without external crates; timestamps are emitted in UTC.

## Testing

Run unit tests and benches:

```bash
cargo test
cargo bench
```

Included tests cover:
- Zero-copy parsing
- Macro formatting and zero-allocation behavior
- File streaming and statistics
- StackBuffer overflow safety
- Date/time formatting (DTime)

## Contributing

Contributions welcome:

1. Fork the repository
2. Create a branch: `git checkout -b feature/your-feature`
3. Commit: `git commit -m "Add feature"`
4. Push: `git push origin feature/your-feature`
5. Open a pull request

Please run tests and keep changes small and focused.

## License

This project is licensed under the MIT License — see LICENSE.

---

Version: 0.3.0  
Language: Rust  
Status: Active Development
