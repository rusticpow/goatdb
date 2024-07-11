//! Stream for operating on files.
use crate::{ReadStream, Result, SeekStream, Stream, WriteStream};
use std::fs::{File, Metadata, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::Arc;

/// Stream that wraps a file.
pub struct FileStream {
    position: u64,
    metadata: Metadata,
    file: Arc<File>,
    mode: FileStreamMode,
    len: u64,
}

pub enum FileStreamMode {
    Read(BufReader<Arc<File>>),
    Write(BufWriter<Arc<File>>),
}

impl FileStream {
    pub fn new_read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;
        let file = Arc::new(file);
        let meta = file.metadata()?;
        Ok(Self {
            len: meta.len(),
            metadata: meta,
            position: 0,
            mode: FileStreamMode::Read(BufReader::new(file.clone())),
            file,
        })
    }

    pub fn new_write<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;
        let file = Arc::new(file);
        let meta = file.metadata()?;
        Ok(Self {
            len: meta.len(),
            metadata: file.metadata()?,
            position: 0,
            mode: FileStreamMode::Write(BufWriter::new(file.clone())),
            file,
        })
    }
}

impl SeekStream for FileStream {
    fn seek(&mut self, to: u64) -> Result<u64> {
        let result = match &mut self.mode {
            FileStreamMode::Read(reader) => Ok(reader.seek(SeekFrom::Start(to))?),
            FileStreamMode::Write(writer) => Ok(writer.seek(SeekFrom::Start(to))?),
        };

        if result.is_ok() {
            self.position = to;
        }

        result
    }

    fn position(&mut self) -> Result<u64> {
        Ok(self.position)
    }

    fn len(&mut self) -> Result<u64> {
        Ok(self.len)
    }
}

impl Read for FileStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if let FileStreamMode::Write(writer) = &mut self.mode {
            // Means our stream mode should be switched to FileStreamMode::Read.
            // All written data should be flushed and BufWriter should be uninitialized.
            writer.flush()?;

            let mut r = BufReader::new(self.file.clone());
            r.seek(SeekFrom::Start(self.position))?;
            self.mode = FileStreamMode::Read(r);
        }

        if let FileStreamMode::Read(reader) = &mut self.mode {
            let size = reader.read(buffer)?;
            self.position += size as u64;
            return Ok(size);
        }

        Err(ErrorKind::Unsupported.into())
    }
}

impl Write for FileStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if let FileStreamMode::Read(_) = &mut self.mode {
            self.mode = FileStreamMode::Write(BufWriter::new(self.file.clone()));
        }

        if let FileStreamMode::Write(writer) = &mut self.mode {
            let size = writer.write(bytes)?;
            self.len += size as u64;
            self.position += size as u64;
            return Ok(size);
        }

        Err(ErrorKind::Unsupported.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let FileStreamMode::Write(writer) = &mut self.mode {
            writer.flush()?;
        }

        // just ignore case when mode != FileStreamMode::Write
        Ok(())
    }
}

impl ReadStream for FileStream {}
impl WriteStream for FileStream {}
impl Stream for FileStream {}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use tempfile::{tempdir, tempfile};

    use crate::{file::FileStream, SeekStream};

    #[test]
    pub fn write() {
        let temp_dir = tempdir().unwrap();
        let mut stream = FileStream::new_write(temp_dir.path().join("write")).expect("file open");

        let buffer: [u8; 6] = [0, 1, 2, 3, 4, 5];
        stream.write_all(&buffer).expect("all should be written");

        assert_eq!(6, stream.len().unwrap());
        assert_eq!(6, stream.position().unwrap());
    }

    #[test]
    pub fn write_read() {
        let temp_dir = tempdir().unwrap();
        let mut stream =
            FileStream::new_write(temp_dir.path().join("write_read")).expect("file open");

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
        let temp_dir = tempdir().unwrap();
        let mut stream =
            FileStream::new_write(temp_dir.path().join("read_to_end")).expect("file open");

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
