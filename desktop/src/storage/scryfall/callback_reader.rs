//! [`Read`]er that hits a callback after each read.

use std::io;
use std::io::Read;

pub struct CallbackReader<F: Fn(usize), R: Read> {
    pub read_bytes: usize,
    pub cb: F,
    pub reader: R,
}

impl<F: Fn(usize), R: Read> Read for CallbackReader<F, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.read_bytes += count;
        (self.cb)(self.read_bytes);
        Ok(count)
    }
}
