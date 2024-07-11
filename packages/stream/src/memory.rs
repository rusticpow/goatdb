//! Stream that reads from and writes to an owned buffer.
use crate::{ReadStream, Result, SeekStream, Stream, WriteStream};
use std::{
    cmp::min,
    io::{Read, Write},
};

/// Stream that wraps an owned buffer.
pub struct MemoryStream {
    buffer: Vec<u8>,
    position: usize,
}

impl MemoryStream {
    /// Create a memory stream.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }
}

impl Default for MemoryStream {
    fn default() -> Self {
        Self::new()
    }
}

impl SeekStream for MemoryStream {
    fn seek(&mut self, to: u64) -> Result<u64> {
        self.position = to.try_into()?;
        Ok(self.position.try_into()?)
    }

    fn position(&mut self) -> Result<u64> {
        Ok(self.position.try_into()?)
    }

    fn len(&mut self) -> Result<u64> {
        Ok(self.buffer.len().try_into()?)
    }
}

impl Read for MemoryStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.position >= self.buffer.len() {
            return Ok(0);
        }

        let source_position = min(self.position + buffer.len(), self.buffer.len());
        (buffer[..(source_position - self.position)])
            .copy_from_slice(&self.buffer[self.position..source_position]);

        let len = source_position - self.position;
        self.position += len;

        Ok(len)
    }
}

impl Write for MemoryStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let bytes_to_end = self.buffer.len() - self.position;
        if bytes.len() > bytes_to_end {
            let bytes_out_of_buffer = bytes.len() - bytes_to_end;
            self.buffer.extend(vec![0u8; bytes_out_of_buffer]);
        }

        self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();

        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl From<Vec<u8>> for MemoryStream {
    fn from(buffer: Vec<u8>) -> Self {
        MemoryStream {
            buffer,
            position: 0,
        }
    }
}

impl From<MemoryStream> for Vec<u8> {
    fn from(val: MemoryStream) -> Self {
        val.buffer
    }
}

impl ReadStream for MemoryStream {}
impl WriteStream for MemoryStream {}
impl Stream for MemoryStream {}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::SeekStream;

    use super::MemoryStream;

    #[test]
    pub fn write() {
        let mut stream = MemoryStream::new();

        let buffer: [u8; 6] = [0, 1, 2, 3, 4, 5];
        stream.write_all(&buffer).expect("all should be written");

        assert_eq!(6, stream.len().unwrap());
        assert_eq!(6, stream.position().unwrap());
    }

    #[test]
    pub fn write_read() {
        let mut stream = MemoryStream::new();

        let buffer: [u8; 6] = [0, 1, 2, 3, 4, 5];
        stream.write_all(&buffer).expect("all should be written");

        stream.seek(0).unwrap();
        assert_eq!(0, stream.position().unwrap());
        assert_eq!(6, stream.len().unwrap());

        let mut buffer_read: [u8; 6] = [0u8; 6];
        stream.read_exact(&mut buffer_read).unwrap();
        assert_eq!(6, stream.position().unwrap());

        assert_eq!(buffer, buffer_read);
    }

    #[test]
    pub fn read_to_end() {
        let mut stream = MemoryStream::new();

        let buffer: [u8; 2] = [42, 10];
        stream.write_all(&buffer).expect("all should be written");

        stream.seek(0).unwrap();
        assert_eq!(0, stream.position().unwrap());
        assert_eq!(2, stream.len().unwrap());

        let mut buffer_read = Vec::new();
        stream.read_to_end(&mut buffer_read).unwrap();

        assert_eq!(buffer, buffer_read.as_slice());
    }
}
