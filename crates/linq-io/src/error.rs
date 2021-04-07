pub use super::usb::error::{Result as UsbResult, UsbError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("failed to parse => {0}")]
    Parser(String),

    #[error("usb communication failure => {0}")]
    Usb(#[from] UsbError),

    #[error("kernel failure => {0}")]
    Kernel(String),

    #[error("device not found => {0}")]
    DeviceNotFound(String),

    #[error("api error => {0}")]
    ApiError(#[from] ApiError),

    #[error("io error => {0}")]
    Io(#[from] std::io::Error),

    #[error("\"impossible\" error => {0}")]
    Impossible(String),

    #[error("Unknown Io Error")]
    Unknown,
}

/// API errors are unique from the library errors. API error is when a device is
/// responding to a request with an error message. Error messages from the
/// device are represented as API errors.
#[derive(Error, Debug)]
pub enum ApiError {
    // #[error("ok")]
    // Linq200,
    #[error("[400] client api protocol error")]
    Linq400,

    #[error("[403] unauthorized")]
    Linq403,

    #[error("[404] resource not found")]
    Linq404,

    #[error("[500] critical server failure")]
    Linq500,

    #[error("[504] please try again later")]
    Linq504,

    #[error("device submitted an unknown error response")]
    LinqUnknown,
}

pub type Result<T> = std::result::Result<T, IoError>;
