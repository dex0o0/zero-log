#[macro_export]
macro_rules! info {
    ($sink:expr => $target:expr, $($arg:tt)*) => {
        $crate::event::log_event($sink, "INFO", $target, format_args!($($arg)*));
    };
    ($target:expr, $($arg:tt)*) => {
        let mut sink = $crate::sink::StdoutSink;
        $crate::event::log_event(&mut sink, "INFO", $target, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($sink:expr => $target:expr, $($arg:tt)*) => {
        $crate::event::log_event($sink, "ERROR", $target, format_args!($($arg)*));
    };
    ($target:expr, $($arg:tt)*) => {
        let mut sink = $crate::sink::StdoutSink;
        $crate::event::log_event(&mut sink, "ERROR", $target, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($sink:expr => $target:expr, $($arg:tt)*) => {
        $crate::event::log_event($sink, "WARN", $target, format_args!($($arg)*));
    };
    ($target:expr, $($arg:tt)*) => {
        let mut sink = $crate::sink::StdoutSink;
        $crate::event::log_event(&mut sink, "WARN", $target, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($sink:expr => $target:expr, $($arg:tt)*) => {
        $crate::event::log_event($sink, "DEBUG", $target, format_args!($($arg)*));
    };
    ($target:expr, $($arg:tt)*) => {
        let mut sink = $crate::sink::StdoutSink;
        $crate::event::log_event(&mut sink, "DEBUG", $target, format_args!($($arg)*));
    };
}
