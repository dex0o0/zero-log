use core::fmt;

pub struct StackBuffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StackBuffer<N> {
    pub fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    pub fn clear(&mut self) {
        self.len = 0
    }
}

impl<const N: usize> fmt::Write for StackBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = N - self.len;

        if bytes.len() > remaining {
            self.buf[self.len..N].copy_from_slice(&bytes[..remaining]);
            self.len = N;
            Err(fmt::Error)
        } else {
            self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            Ok(())
        }
    }
}

impl<const N: usize> Default for StackBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}
