use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Try to read after end of file")]
    UnexpectedEof,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}
