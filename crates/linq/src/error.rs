use linq_io::error::IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinqError {
    #[error("linq transport error => {0}")]
    Io(#[from] IoError),

    #[error("failed to parse => {0}")]
    Parser(String),

    #[error("io error => {0}")]
    StdIo(#[from] std::io::Error),

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, LinqError>;
