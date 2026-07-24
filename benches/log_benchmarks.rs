use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use zero_log::{FileStreamer, LogEntry};

fn bench_log_parsing(c: &mut Criterion) {
    let log_line = "[1721580000] [ERROR] [auth] Failed login attempt from IP 192.168.1.100";

    c.bench_function("parse_single_log_line", |b| {
        b.iter(|| LogEntry::parse(black_box(log_line)));
    });
}

fn bench_file_streaming(c: &mut Criterion) {
    let test_path = "bench_tmp.log";
    {
        let mut file = File::create(test_path).unwrap();
        for i in 0..100_000 {
            let level = if i % 10 == 0 { "ERROR" } else { "INFO" };
            writeln!(
                file,
                "[{}] [{}] [sys] Processing event number {}",
                i, level, i
            )
            .unwrap();
        }
    }
    c.bench_function("stream_and_analyze_100k_lines", |b| {
        b.iter(|| {
            FileStreamer::analyze_file(black_box(test_path)).unwrap();
        });
    });
    let _ = std::fs::remove_file(test_path);
}
criterion_group!(benches, bench_log_parsing, bench_file_streaming);
criterion_main!(benches);
