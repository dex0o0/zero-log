# zero-log 📝

A lightweight and efficient Rust library for logging and log analysis, with a focus on memory efficiency and fast processing using zero-copy parsing techniques.

## ✨ Key Features

- 🚀 **High Performance** - Zero-copy parsing for minimal memory usage
- 📊 **Log Analysis** - Parse and analyze stored logs efficiently
- 🎯 **Multiple Log Levels** - Support for ERROR, WARN, INFO, and DEBUG
- 💾 **Flexible Output** - Write logs to files or standard output
- 🔍 **Smart Filtering** - Filter logs by level and other criteria
- 📈 **Statistics** - Count and analyze log entries

## 🛠️ Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zero-log = { path = "./zero-log" }
```

## 📖 Quick Start

### 1. Create a Logger and Write Logs

```rust
use zero_log::{Logger, LogLevel};

fn main() -> std::io::Result<()> {
    // Write logs to a file
    let mut logger = Logger::new("app.log")?;
    
    logger.info("auth", "User logged in")?;
    logger.error("db", "Database connection failed")?;
    
    Ok(())
}
```

### 2. Use Convenient Macros

```rust
use zero_log::{Logger, info, error, warn, debug};

fn main() -> std::io::Result<()> {
    let mut logger = Logger::new("app.log")?;
    
    let user_id = 42;
    let ip = "192.168.1.50";
    
    info!(logger, "auth", "User {} logged in", user_id);
    error!(logger, "net", "Connection failed from IP: {}", ip);
    warn!(logger, "sys", "Memory usage is high");
    debug!(logger, "db", "Query executed");
    
    Ok(())
}
```

### 3. Log to Console (stdout)

```rust
use zero_log::Logger;

fn main() -> std::io::Result<()> {
    let mut logger = Logger::stdout();
    
    logger.info("app", "Application started")?;
    
    Ok(())
}
```

### 4. Parse and Analyze Logs

```rust
use zero_log::{LogAnalyzer, LogLevel};

fn main() -> std::io::Result<()> {
    let log_content = std::fs::read_to_string("app.log")?;
    let analyzer = LogAnalyzer::new(&log_content);
    
    // Filter only ERROR logs
    for entry in analyzer.filter_by_level(LogLevel::Error) {
        println!("Error: {} - {}", entry.target, entry.message);
    }
    
    Ok(())
}
```

### 5. Get Log Statistics

```rust
use zero_log::FileStreamer;

fn main() -> std::io::Result<()> {
    let stats = FileStreamer::analyze_file("app.log")?;
    
    println!("Total logs: {}", stats.total_entries);
    println!("Errors: {}", stats.error_count);
    println!("Warnings: {}", stats.warn_count);
    println!("Info: {}", stats.info_count);
    println!("Debug: {}", stats.debug_count);
    
    Ok(())
}
```

### 6. Stream and Process Logs

```rust
use zero_log::FileStreamer;

fn main() -> std::io::Result<()> {
    // Process logs line by line
    FileStreamer::stream("app.log", |entry| {
        if entry.level == "ERROR" {
            println!("⚠️ Error: {}", entry.message);
        }
    })?;
    
    Ok(())
}
```

## 📋 Log Format

Logs follow this standard format:

```
[timestamp] [level] [target] message
```

**Example:**
```
[1721580000] [ERROR] [auth_service] Invalid password attempt
[1721580001] [INFO] [server] Server started successfully
[1721580002] [WARN] [db] Connection pool nearly exhausted
```

## 🏗️ Architecture

### Main Components

| Component | Description |
|-----------|-------------|
| **LogLevel** | Enumeration for log levels: ERROR, WARN, INFO, DEBUG |
| **Logger** | Main struct for writing new logs to file or stdout |
| **LogEntry** | Represents a single parsed log line |
| **LogAnalyzer** | Analyzes and filters a collection of logs |
| **LogStats** | Stores statistics about logs |
| **FileStreamer** | Streams log files for processing |

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Tests included:
- ✅ Zero-copy parsing test
- ✅ Log filtering by level
- ✅ File streaming and statistics
- ✅ Logging macros

## 💡 Complete Example

```rust
use zero_log::{Logger, FileStreamer, LogAnalyzer, LogLevel, info, error};

fn main() -> std::io::Result<()> {
    // 1. Write some logs
    let mut logger = Logger::new("server.log")?;
    
    info!(logger, "server", "Server started");
    error!(logger, "db", "Database connection error");
    info!(logger, "auth", "User authentication successful");
    
    // 2. Analyze the log file
    let content = std::fs::read_to_string("server.log")?;
    let analyzer = LogAnalyzer::new(&content);
    
    // 3. Filter and display errors
    println!("\n--- Errors Only ---");
    for entry in analyzer.filter_by_level(LogLevel::Error) {
        println!("[{}] {}: {}", entry.timestamp, entry.target, entry.message);
    }
    
    // 4. Get statistics
    let stats = FileStreamer::analyze_file("server.log")?;
    println!("\n--- Statistics ---");
    println!("Total entries: {}", stats.total_entries);
    println!("Errors: {}", stats.error_count);
    println!("Info messages: {}", stats.info_count);
    
    Ok(())
}
```

## 📚 API Documentation

### Logger

- `Logger::new(path)` - Create a logger that writes to a file
- `Logger::stdout()` - Create a logger that writes to console
- `logger.log(level, target, message)` - Write a log entry
- `logger.info(target, message)` - Write an INFO level log
- `logger.error(target, message)` - Write an ERROR level log

### LogAnalyzer

- `LogAnalyzer::new(data)` - Create an analyzer from log content
- `analyzer.entries()` - Get all parsed log entries
- `analyzer.filter_by_level(level)` - Filter logs by level

### FileStreamer

- `FileStreamer::stream(path, callback)` - Process log file line by line
- `FileStreamer::analyze_file(path)` - Get statistics for a log file

### Macros

- `info!(logger, target, message)` - Info level log
- `error!(logger, target, message)` - Error level log
- `warn!(logger, target, message)` - Warning level log
- `debug!(logger, target, message)` - Debug level log

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 👨‍💻 Author

Created by [@dex0o0](https://github.com/dex0o0)

## 🤝 Contributing

Contributions are welcome! Here's how:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add YourFeature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a Pull Request

## ⭐ Show Your Support

If you find this project helpful, please give it a ⭐!

---

**Version:** 0.2.0  
**Language:** Rust 🦀  
**Status:** Active Development
