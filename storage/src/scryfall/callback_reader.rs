//! [`Read`]er that hits a callback after each read.

use std::io;
use std::io::Read;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

pub trait ProgressCallback {
    fn call(&mut self, bytes_read: usize, elapsed_time: Duration);
}

pub struct CallbackReader<F: ProgressCallback, R: Read> {
    start: LazyLock<Instant>,
    read_bytes: usize,
    cb: F,
    reader: R,
}

impl<F: ProgressCallback, R: Read> CallbackReader<F, R> {
    pub fn new(cb: F, reader: R) -> Self {
        CallbackReader {
            start: LazyLock::new(Instant::now),
            read_bytes: 0,
            cb,
            reader,
        }
    }
}

impl<F: ProgressCallback, R: Read> Read for CallbackReader<F, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.read_bytes += count;
        self.cb.call(self.read_bytes, self.start.elapsed());
        Ok(count)
    }
}
