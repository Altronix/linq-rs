use super::request::*;
use super::update::*;
use super::usb::usb::Usb;
use super::usb::{UsbChannel, UsbMetadata};
use crate::channel::Channel;
use crate::error::{IoError, Result as IoResult};
use futures::future::LocalBoxFuture;
use futures::prelude::*;
use futures::stream;
use futures::stream::LocalBoxStream;
use futures::Stream;
use linq_util::gen_log_helpers;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::future::Future;
use std::io::prelude::*;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

gen_log_helpers!("COM");

/// Main IO context (manages thread workers and Map of all connected devices)
pub struct Io {
    /// Usb Thread manager
    usb: Arc<Usb>,
    /// Map of all connected devices
    channels: HashMap<String, Box<dyn Channel>>,
}

impl Io {
    /// Create a new Io object to manage communication channels.
    pub fn new() -> Self {
        Io {
            usb: Arc::new(Usb::new()),
            channels: HashMap::new(),
        }
    }

    /// Scan USB port, adding each supported product into channel
    pub fn scan<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = IoResult<Vec<UsbMetadata>>> + 'a>> {
        Box::pin(async move {
            let mut v: Vec<UsbMetadata> = vec![];
            self.usb.scan().await?.into_iter().for_each(|x| {
                let serial = x.serial.clone();
                let ch = Box::new(UsbChannel::new(Arc::clone(&self.usb), x));
                v.push(ch.meta.clone());
                self.channels.insert(serial, ch);
            });
            Ok(v)
        })
    }

    /// Print out some version information
    pub fn version<'a>() -> &'static str {
        Usb::version()
    }

    /// Delete all channels, free any threads, etc. Essentially an explicit
    /// destructor. It is recommend to join threads explicitly as opposed to in
    /// a drop routine. (If you forget to close then you will see panic on
    /// program exit, so your program will always close correctly, or panic).
    pub fn close(&mut self) -> IoResult<()> {
        self.channels.clear();
        Arc::get_mut(&mut self.usb)
            .ok_or(IoError::Impossible("dangling reference to usb".into()))?
            .close()
            .map_err(|_| IoError::Kernel("failed to join thread".into()))
    }

    /// Get a box of varius metadata
    pub fn meta(&self) -> IoResult<Vec<UsbMetadata>> {
        self.channels
            .iter()
            .map(|x| {
                // TODO instead of returning only UsbMeta, figure best way to
                //      distinguish different meta types
                let meta = serde_json::from_str::<UsbMetadata>(&(*x.1).meta())
                    .map_err(|x| IoError::Parser(x.to_string()));
                meta
            })
            .collect()
    }

    /// Update a device from a file on the fs
    pub fn update_file_path<'a>(
        &'a self,
        serial: &'a str,
        path: &'a str,
        image: u8, // <--- TODO deprecate need to move updates to channel trait
    ) -> IoResult<impl Stream<Item = IoResult<(usize, usize)>> + 'a> {
        let update = DashboardUpdatePackets::parse_file(path)?;
        Ok(self.update(serial, update, image))
    }

    /// Send an update to device
    pub fn update<'a>(
        &'a self,
        sid: &'a str,
        pack: DashboardUpdatePackets,
        image: u8, // TODO deprecate need to move updates to channel trait
    ) -> impl Stream<Item = IoResult<(usize, usize)>> + 'a {
        let image = if image == 0 { pack.0 } else { pack.1 };
        let s0 = self.requests_stream(sid, image, move |_, n, t| (n, t));
        stream::iter(vec![s0]).flatten()
    }

    /// Convert an array of requests into a stream
    /// TODO see if we can implement stream trait for Requests instead...
    fn requests_stream<'a, F, T>(
        &'a self,
        serial: &'a str,
        requests: Vec<Request>,
        mut mapper: F,
    ) -> LocalBoxStream<'a, IoResult<T>>
    where
        F: FnMut(String, usize, usize) -> T + 'a + Copy,
    {
        let total = requests.len();
        stream::unfold(requests, move |mut requests| async move {
            match requests.pop() {
                Some(r) => self.request(serial, r).await.map_or_else(
                    |e| Some((Err(e), vec![])),
                    |s| {
                        let n = total - requests.len();
                        Some((Ok(mapper(s, n, total)), requests))
                    },
                ),
                _ => None,
            }
        })
        .boxed_local()
    }

    /// Send a get request
    pub async fn get(&self, serial: &str, path: &str) -> IoResult<String> {
        self.request(serial, Request::get(path))
            .await
            .map_err(|x| x.into())
    }

    /// Send a request to a device with serial number [serial]
    pub fn request<'a>(
        &'a self,
        serial: &'a str,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = IoResult<String>> + 'a>> {
        info!("{}", request);
        let ch = self
            .channels
            .get(serial)
            .ok_or(IoError::DeviceNotFound(serial.to_owned()));
        Box::pin(async move { ch?.request_raw(serial, request).await })
    }
}
