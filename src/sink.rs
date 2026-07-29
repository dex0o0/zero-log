use std::fs::File;
use std::io::{self, Write};

pub trait LogSink {
    fn write_log(&mut self, data: &[u8]) -> io::Result<()>;

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct StdoutSink;

impl LogSink for StdoutSink {
    fn write_log(&mut self, data: &[u8]) -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(data)?;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

pub struct FileSink {
    file: File,
}

impl FileSink {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl LogSink for FileSink {
    fn write_log(&mut self, data: &[u8]) -> io::Result<()> {
        self.file.write_all(data)?;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

#[cfg(test)]
pub struct MemorySink<const N: usize> {
    buf: [u8; N],
    len: usize,
}

#[cfg(test)]
impl<const N: usize> MemorySink<N> {
    pub fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }
}

#[cfg(test)]
impl<const N: usize> LogSink for MemorySink<N> {
    fn write_log(&mut self, data: &[u8]) -> io::Result<()> {
        let remaning = N - self.len;
        let to_copy = data.len().min(remaning);
        self.buf[self.len..self.len + to_copy].copy_from_slice(&data[..to_copy]);
        self.len += to_copy;
        Ok(())
    }
}
