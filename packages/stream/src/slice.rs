//! Stream that reads from a slice of bytes.
use crate::{ReadStream, Result, SeekStream};
use std::{
    cmp::min,
    io::{Error, ErrorKind, Read},
};

/// Stream that wraps a slice of bytes.
pub struct SliceStream<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> SliceStream<'a> {
    /// Create a slice stream.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }
}

impl SeekStream for SliceStream<'_> {
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

impl Read for SliceStream<'_> {
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

impl ReadStream for SliceStream<'_> {}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::SeekStream;

    use super::SliceStream;

    #[test]
    pub fn read_repeated() {
        let buffer: [u8; 6] = [0, 1, 2, 3, 4, 5];
        let mut stream = SliceStream::new(&buffer);

        stream.seek(0).unwrap();
        assert_eq!(0, stream.position().unwrap());
        assert_eq!(6, stream.len().unwrap());

        let mut buffer_read: [u8; 6] = [0u8; 6];
        stream.read_exact(&mut buffer_read).unwrap();
        assert_eq!(6, stream.position().unwrap());

        assert_eq!(buffer, buffer_read);
    }
}
