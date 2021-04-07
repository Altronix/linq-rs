/// A ReaderWriter provides low level byte transfer. This abstraction is not
/// entirely necessary except that it is only useful to stub out a concret
/// implementation in order to facilitate testing.
use crate::error::*;
use crate::request::Request;
use core::pin::Pin;
use futures::stream::BoxStream;
use serde::de::DeserializeOwned;
use std::future::Future;

/// A Channel is able to make async requests and describe it self
pub trait Channel: AsyncRequester + Meta {}

/// A Meta trait is able to describe itself as a JSON string
pub trait Meta {
    /// Implementing this trait must provide a JSON string
    fn meta(&self) -> String;
}

/// An AsyncRequester is same as a SyncRequest excepts returns a Future!
pub trait AsyncRequester {
    /*
    // NOTE this generic can't be inside a trait object!
    fn request<'a, R>(
        &self,
        serial: &'a str,
        r: Request,
    ) -> Pin<Box<dyn Future<Output = Result<R>>>>
    where
        Self: Sized,
        R: DeserializeOwned + Send,
    {
        let result = self.request_raw(serial, r);
        Box::pin(async move {
            let response = result.await?;
            serde_json::from_str::<R>(&response)
                .map_err(|x| IoError::Parser(x.to_string()))
        })
    }
    */

    fn request_raw<'a>(
        &'a self,
        serial: &'a str,
        r: Request,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + 'a>>;
}

pub trait AsyncUpdater {
    fn update<'a>(
        &'a self,
        serial: &'a str,
        update: &'a str,
    ) -> Result<BoxStream<'a, (usize, usize)>>;
}
