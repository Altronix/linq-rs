use std::io::Error;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewError {
    #[error("io error => {0}")]
    Io(#[from] std::io::Error),

    #[error("utf8 => {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("ipc => {0}")]
    Ipc(String),

    #[error("impossible =>{0}")]
    Impossible(String),
}

pub type Result<T> = std::result::Result<T, ViewError>;
