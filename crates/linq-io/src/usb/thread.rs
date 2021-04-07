use super::binding::{Binding, Driver};
use super::metadata::UsbMetadata;
use crate::error::*;
use crate::request::Request;
use futures::channel::oneshot::Sender as OneshotSender;
use std::sync::mpsc::Receiver;

// TODO scan should return some usb device descriptions
pub struct UsbRequestScan {
    pub response: OneshotSender<Result<Vec<UsbMetadata>>>,
}
pub struct UsbRequestDevice {
    pub response: OneshotSender<Result<String>>,
    pub serial: String,
    pub request: Request,
    pub driver: Driver,
}
pub enum UsbRequest {
    Scan(UsbRequestScan),
    Device(UsbRequestDevice),
    Close,
}

/// This helper routine converts our result so all our match arms match
/// when dispatching requests
fn scan(binding: &mut Binding, request: UsbRequestScan) -> Result<()> {
    request
        .response
        .send(binding.scan())
        .map_err(|_| IoError::Unknown)?;
    Ok(())
}

/// This helper routine converts our result so all our match arms match
/// when dispatching requests
fn request(binding: &mut Binding, request: UsbRequestDevice) -> Result<()> {
    request
        .response
        .send(binding.request(&request.serial, request.request, request.driver))
        .map_err(|_| IoError::Unknown)?;
    Ok(())
}

/// Main usb worker. Simply receives requests and dispaches them to the usb
/// driver. We need this thread to provide a non blocking api for usb comm
pub fn usb_thread(rx: Receiver<UsbRequest>) {
    let mut binding = Binding::new();
    for r in rx.iter() {
        let result = match r {
            UsbRequest::Scan(r) => scan(&mut binding, r),
            UsbRequest::Device(r) => request(&mut binding, r),
            UsbRequest::Close => break,
        };
        if result.is_err() {
            break;
        }
    }
}
