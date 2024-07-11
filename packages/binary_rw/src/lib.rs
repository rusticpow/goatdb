pub mod file;
pub mod memory;
pub mod slice;
pub mod error;

use std::{
    io::{Read, Write},
};

use error::StreamError;

pub type Result<T> = std::result::Result<T, StreamError>;

/// Trait for streams that can seek.
pub trait SeekStream {
    /// Seek to a position.
    fn seek(&mut self, to: usize) -> Result<usize>;
    /// Get the current position.
    fn tell(&mut self) -> Result<usize>;
    /// Get the length of the stream.
    fn len(&self) -> Result<usize>;
}

/// Trait for a readable stream.
pub trait ReadStream: Read + SeekStream {}

/// Trait for a writable stream.
pub trait WriteStream: Write + SeekStream {}
