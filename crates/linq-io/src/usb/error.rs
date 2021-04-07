use crate::error::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsbError {
    #[error("protocol violation => {0}")]
    Protocol(String),

    #[error("low level usbh driver error => {0}")]
    Usbh(i32, &'static str),

    #[error("low level libusb binding error => {0} {1}")]
    Libusb(i32, &'static str),

    #[error("device not found => {0}")]
    DeviceNotFound(String),

    #[error("failed to parse => {0}")]
    Parser(String),

    #[error("unknown")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, UsbError>;
