use super::binding::{Binding, Driver};
use crate::error::*;
use crate::request::Request;
use futures::channel::oneshot;
use std::future::Future;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use super::metadata::UsbMetadata;
use super::thread::*;

/// Our Usb Binding is Syncronous. We delegate it to it's own thread
pub struct Usb {
    tx: Sender<UsbRequest>,
    join_handle: Option<JoinHandle<()>>,
}

/// We wrap our usb binding with a "Manager" class that provides async api
impl Usb {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let join_handle = std::thread::spawn(move || usb_thread(rx));
        let join_handle = Some(join_handle);
        Usb { tx, join_handle }
    }

    /// get binding version
    pub fn version() -> &'static str {
        Binding::version()
    }

    /// Async wrapper for USB driver call to scanning USB devices
    pub fn scan(&self) -> impl Future<Output = Result<Vec<UsbMetadata>>> {
        let (tx, rx) = oneshot::channel::<Result<Vec<UsbMetadata>>>();
        self.tx
            .send(UsbRequest::Scan(UsbRequestScan { response: tx }))
            .expect("Usb channel has been closed!");
        async { rx.await.map_err(|_| IoError::Unknown)? }
    }

    /// Async wrapper for request
    pub fn request<'a>(
        &self,
        serial: &'a str,
        request: Request,
        driver: Driver,
    ) -> impl Future<Output = Result<String>> {
        let (tx, rx) = oneshot::channel::<Result<String>>();
        self.tx
            .send(UsbRequest::Device(UsbRequestDevice {
                request,
                serial: serial.to_owned(),
                response: tx,
                driver,
            }))
            .expect("Usb channel has been closed!");
        async { rx.await.map_err(|_| IoError::Unknown)? }
    }

    /// We explicitly must free the USB device
    pub fn close(&mut self) -> std::thread::Result<()> {
        self.tx
            .send(UsbRequest::Close)
            .expect("Usb channel has been closed!");
        match self.join_handle.take() {
            Some(h) => h.join(),
            None => Ok(()),
        }
    }
}

impl Drop for Usb {
    fn drop(&mut self) {
        if self.join_handle.is_some() {
            panic!("You must close USB when you are finished!");
        }
    }
}
