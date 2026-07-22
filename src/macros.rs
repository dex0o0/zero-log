#[macro_export]
macro_rules! log_msg {
    ($logger:expr,$level:expr,$target:expr,$($arg:tt)*) => {
        let msg = format!($($arg)*);
        let _ = $logger.log($level,$target,&msg);
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr,$target:expr,$($arg:tt)*) => {
        $crate::log_msg!($logger,$crate::LogLevel::Info,$target,$($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr,$target:expr,$($arg:tt)*) => {
        $crate::log_msg!($logger,$crate::LogLevel::Error,$target,$($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr,$target:expr,$($arg:tt)*) => {
        $crate::log_msg!($logger,$crate::LogLevel::Warn,$target,$($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr,$target:expr,$($arg:tt)*) => {
        $crate::log_msg!($logger,$crate::LogLevel::Debug,$target,$($arg)*);
    };
}
