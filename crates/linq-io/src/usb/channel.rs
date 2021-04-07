use super::metadata::UsbMetadata;
use super::usb::Usb;
use crate::channel::{AsyncRequester, Channel, Meta};
use crate::error::*;
use crate::request::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
struct ErrorPacket {
    error: u32,
}

/// Helper for storing Open channels
pub struct UsbChannel {
    pub meta: UsbMetadata,
    usb: Arc<Usb>,
}

impl UsbChannel {
    /// Create a new USB channel instance (requires handle to USB context)
    pub fn new<'a>(usb: Arc<Usb>, meta: UsbMetadata) -> Self {
        UsbChannel { usb, meta }
    }

    fn send<'str>(
        &self,
        _serial: &'str str, // Ignored (see note below)
        r: Request,
    ) -> Pin<Box<dyn Future<Output = Result<String>>>>
    where
        Self: Sized,
    {
        // NOTE that serial and sid are not in sync with eachother because
        //      certain USB devices have inpropper USB Descriptors. Serial
        //      is the sane way to reach the device per our front facing API.
        //      However, sid is needed to actually comunicate with the device.
        //      So this routine maps correct serial to sid with out bothering
        //      caller. Should USB devices fix their descriptors, then we can
        //      forward the regular serial IE: replace meta.sid w/ serial
        //      as first argument to request.
        let f = self.usb.request(&self.meta.sid, r, self.meta.driver);
        Box::pin(async move {
            let response = f.await?;
            Ok(response)
        })
    }
}

impl Channel for UsbChannel {}
impl AsyncRequester for UsbChannel {
    fn request_raw<'a>(
        &'a self,
        _serial: &'a str, // Ignored (see note below)
        r: Request,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + 'a>>
    where
        Self: Sized,
    {
        let f = self.usb.request(&self.meta.sid, r, self.meta.driver);
        Box::pin(async move {
            let response = f.await?;
            Ok(response)
        })
    }
}

impl Meta for UsbChannel {
    fn meta(&self) -> String {
        serde_json::to_string(&self.meta).unwrap()
    }
}
