use core::fmt::{self, Write as FmtWrite};
use std::cell::RefCell;

use crate::buffer::StackBuffer;
use crate::sink::LogSink;
use crate::time_date::DTime;

thread_local! {
    static LOG_BUFFER: RefCell<StackBuffer<4096>> = RefCell::new(StackBuffer::new());
}

pub fn log_event<S: LogSink>(sink: &mut S, level: &str, target: &str, args: fmt::Arguments) {
    let now = DTime::now();

    LOG_BUFFER.with(|buf| {
        let Ok(mut buffer) = buf.try_borrow_mut() else {
            return;
        };

        buffer.clear();

        if writeln!(buffer, "[{}] [{}] [{}] {}", now, level, target, args).is_ok() {
            let _ = sink.write_log(buffer.as_bytes());
        }
    });
}
