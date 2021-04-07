use super::drivers::driver::{Reader, ReaderWriter, Writer};
use super::drivers::k64;
use super::drivers::m5;
use super::metadata::{Summary, UsbMetadata};
use crate::error::*;
use crate::request::Request;
use linq_sys::*;
use linq_util::lformat;
use log::{debug, error, info, trace, warn};
use std::ffi::{CStr, CString};

const PID_K64: u32 = 0x0020;
const PID_M5: u32 = 0x4444;

/// Helper type when forwarding requests to the correct driver
#[derive(Copy, Clone)]
pub struct Driver(fn(ctx: &Binding, &str, Request) -> Result<String>);

/// Need custom debug implementation
impl std::fmt::Debug for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// NOTE we only impelement default for serde serializer...
impl Default for Driver {
    fn default() -> Self {
        Driver(k64::request_raw)
    }
}

/// Our binding needs a c compatible callback to redirect our logs
unsafe extern "C" fn logger(s: *mut linq_sys::log_callback_s) {
    // TODO should batch these
    let file = CStr::from_ptr((*s).file).to_str().unwrap();
    let level = CStr::from_ptr((*s).level).to_str().unwrap();
    let cat = CStr::from_ptr((*s).category).to_str().unwrap();
    let mesg = CStr::from_ptr((*s).message.as_ptr()).to_str().unwrap();
    let line = (*s).line;
    match level {
        "TRACE" => trace!("{}", lformat!(file, line, cat, mesg)),
        "FATAL" => error!("{}", lformat!(file, line, cat, mesg)),
        "ERROR" => error!("{}", lformat!(file, line, cat, mesg)),
        "WARN " => warn!("{}", lformat!(file, line, cat, mesg)),
        "DEBUG" => debug!("{}", lformat!(file, line, cat, mesg)),
        "INFO " => info!("{}", lformat!(file, line, cat, mesg)),
        _ => panic!("bad logs from usbh binding!"),
    }
}

/// Super thing wrapper around our binding. See Usb{...} for rust ergonomic api
pub struct Binding {
    binding: *mut usbh_s,
}

/// This helper class provide rust like API for our C binding
impl Binding {
    /// Create our void pointer
    pub fn new() -> Self {
        let binding = unsafe {
            linq_sys::usbh_log_fn_set(Some(logger), std::ptr::null_mut());
            linq_sys::usbh_create()
        };
        Binding { binding }
    }

    /// get strerror of most recent error
    pub fn strerror(e: i32) -> &'static str {
        unsafe { CStr::from_ptr(linq_sys::usbh_strerror(e)).to_str().unwrap() }
    }

    /// We have some return value and a binding error code. Convert to result
    pub fn into_result(e: i32) -> Result<i32> {
        let msg = Self::strerror(e);
        match e {
            E_LINQ_ERROR_LINQ_ERROR_OK => Ok(e),
            E_LINQ_ERROR_LINQ_ERROR_LIBUSB => {
                Err(UsbError::Libusb(e, msg).into())
            }
            _ => Err(UsbError::Usbh(e, msg).into()),
        }
    }

    /// get binding version
    pub fn version() -> &'static str {
        unsafe { CStr::from_ptr(linq_sys::usbh_version()).to_str().unwrap() }
    }

    /// Get a JSON description of all connected devices from our binding
    pub fn summary_raw(&self) -> String {
        unsafe {
            let mut c = linq_sys::usbh_summary_alloc(self.binding);
            let s = CStr::from_ptr(c).to_str().unwrap().to_owned();
            linq_sys::usbh_summary_free(&mut c);
            s
        }
    }

    /// Get a string and parse it as an array of connected devices
    pub fn scan(&mut self) -> Result<Vec<UsbMetadata>> {
        let e = unsafe { linq_sys::usbh_scan(self.binding) };
        Self::into_result(e as i32)?;
        serde_json::from_str::<Vec<Summary>>(&self.summary_raw())
            .map_err(|x| IoError::Parser(x.to_string()))?
            .iter()
            .map(|x| {
                let (pid, sid) = (x.product, &x.serial);
                let (serial, driver) = match x.product {
                    PID_K64 => k64::open(self, &sid),
                    PID_M5 => m5::open(self, &sid),
                    _ => {
                        let e = format!("invalid pid [{}]", pid).to_owned();
                        Err(UsbError::Protocol(e).into())
                    }
                }?;
                Ok(UsbMetadata::new(&serial, Driver(driver), &x))
            })
            .collect()
    }

    /// Send a request to a usb channel
    pub fn request<'a>(
        &self,
        serial: &'a str,
        request: Request,
        driver: Driver,
    ) -> Result<String> {
        driver.0(self, serial, request)
    }

    /// Translate our Rust types into C and send to binding
    pub fn send<'a>(&self, name: &'a str, s: &[u8]) -> Result<usize> {
        let c = CString::new(name).unwrap();
        let e = unsafe {
            linq_sys::usbh_send(
                self.binding,
                c.as_ptr(),
                s.as_ptr(),
                s.len() as u32,
            )
        };
        Self::into_result(e)?;
        Ok(s.len())
    }

    /// Translate C types and receive from our rust binding
    pub fn recv<'a>(&self, name: &'a str, bytes: &mut [u8]) -> Result<usize> {
        self.recv_exact(name, bytes, bytes.len())
    }

    /// Recv except the length is explicit and "smaller" then bytes.len()
    pub fn recv_exact<'a>(
        &self,
        name: &'a str,
        bytes: &mut [u8],
        len: usize,
    ) -> Result<usize> {
        let c = CString::new(name).unwrap();
        let mut len = len as u32;
        let e = unsafe {
            linq_sys::usbh_recv(
                self.binding,
                c.as_ptr(),
                bytes.as_mut_ptr(),
                &mut len,
                2000 as u32,
            )
        };
        Self::into_result(e)?;
        Ok(len as usize)
    }
}

/// Free our binding memory that rust is not aware of and can't drop for us
impl Drop for Binding {
    fn drop(&mut self) {
        unsafe { linq_sys::usbh_destroy(&mut self.binding) };
    }
}

/// Concrete implementation for Usb Rx/Tx
impl ReaderWriter for Binding {}

/// Concrete implementation of USB Tx
impl Writer for Binding {
    fn write<'a>(&self, s: &'a str, bytes: &[u8]) -> Result<usize> {
        self.send(s, bytes)
    }
}

/// Concrete implementation of USB Rx
impl Reader for Binding {
    fn read<'a>(&self, s: &'a str, bytes: &mut [u8]) -> Result<usize> {
        self.recv(s, bytes)
    }
}
