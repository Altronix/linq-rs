use super::super::driver::ReaderWriter;
use crate::error::*;
use crate::Request;
use serde::de::DeserializeOwned;

/// Helper method to parse caller data if type is serializable
pub fn request<R: DeserializeOwned>(
    ctx: &impl ReaderWriter,
    sid: &str,
    r: Request,
) -> Result<R> {
    request_raw(ctx, &sid, Request::get("/ATX/about")).and_then(|r| {
        serde_json::from_str::<R>(&r)
            .map_err(|x| UsbError::Parser(x.to_string()).into())
    })
}

pub fn request_raw(
    ctx: &impl ReaderWriter,
    sid: &str,
    r: Request,
) -> Result<String> {
    Ok("".to_owned())
}

/// Return a driver handle for a K64 USB device
pub fn open<T: ReaderWriter>(
    ctx: &T,
    sid: &str,
) -> Result<(String, fn(ctx: &T, sid: &str, r: Request) -> Result<String>)> {
    Ok(("".to_owned(), request_raw))
}
