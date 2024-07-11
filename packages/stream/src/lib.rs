pub mod error;
pub mod file;
pub mod memory;
pub mod slice;

use std::io::{Read, Write};

use error::StreamError;

pub type Result<T> = std::result::Result<T, StreamError>;

/// Trait for streams that can seek.
pub trait SeekStream {
    /// Seek to a position.
    fn seek(&mut self, to: u64) -> Result<u64>;
    /// Get the current position.
    fn position(&mut self) -> Result<u64>;
    /// Get the length of the stream.
    fn len(&mut self) -> Result<u64>;
}

/// Trait for a readable stream.
pub trait ReadStream: Read + SeekStream {}

/// Trait for a writable stream.
pub trait WriteStream: Write + SeekStream {}

pub trait Stream: ReadStream + WriteStream {}
