use super::super::driver::{Reader, ReaderWriter};
use super::packet;
use crate::error::{ApiError, IoError, Result, UsbError};
use crate::request::Request;
use linq_db::k64::{About, AboutResponse};
use linq_util::log::*;
use packet::{ACK, IO_SIZE, PREAMBLE};
use serde::de::DeserializeOwned;
use serde_json::Value;

gen_log_helpers!("K64");

pub const MAX_RETRY: u8 = 3;

/// Helper to make sure we have a valid ack
fn read_ack<'a>(ctx: &impl Reader, s: &'a str) -> Result<()> {
    debug!("[{}] read_ack", s);
    let l: usize;
    let mut incoming: [u8; IO_SIZE] = [0; IO_SIZE];
    l = ctx.read(s, &mut incoming)?;
    if !(l == IO_SIZE && (ACK == incoming)) {
        let e = format!("[{}] failed to receive acknowledge!", s);
        warn!("{}", e);
        Err(UsbError::Protocol(e.to_owned()).into())
    } else {
        Ok(())
    }
}

/// NOTE The USB on the OS level can have cached incoming bytes. This will
///      flush out what ever is pending so we can start from a fresh state
fn flush(ctx: &impl Reader, s: &str) {
    let mut incoming: [u8; IO_SIZE] = [0; IO_SIZE];
    loop {
        let l = ctx.read(s, &mut incoming);
        match l {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}

/// Helper to make sure we have a valid preamble
fn read_preamble<'a>(ctx: &impl Reader, s: &'a str) -> Result<()> {
    debug!("[{}] read_preamble", s);
    let l: usize;
    let mut incoming: [u8; IO_SIZE] = [0; IO_SIZE];
    l = ctx.read(s, &mut incoming)?;
    if !(l == IO_SIZE && (PREAMBLE == incoming)) {
        let e = format!("[{}] failed to receive preamble!", s);
        warn!("{}", e);
        Err(UsbError::Protocol(e.to_owned()).into())
    } else {
        Ok(())
    }
}

/// Helper method to parse caller data if type is serializable
pub fn request<R: DeserializeOwned>(
    ctx: &impl ReaderWriter,
    sid: &str,
    r: Request,
) -> Result<R> {
    request_raw(ctx, &sid, r).and_then(|r| {
        serde_json::from_str::<R>(&r)
            .map_err(|x| UsbError::Parser(x.to_string()).into())
    })
}

/// Attempt to make a request
pub fn make_request(
    ctx: &impl ReaderWriter,
    sid: &str,
    r: &Request,
) -> Result<String> {
    let mut incoming: [u8; IO_SIZE] = [0; IO_SIZE];
    let (len, packets) = packet::from_request(r);
    ctx.write(&sid, &PREAMBLE)?;
    read_ack(ctx, &sid)?;
    ctx.write(&sid, &len)?;
    for p in packets {
        read_ack(ctx, &sid)?;
        ctx.write(&sid, &p)?;
    }
    ctx.read(&sid, &mut incoming)?;
    if incoming != PREAMBLE {
        // Short mode
        debug!("{}", "received short mode packet");
        let mut v = vec![[0; IO_SIZE]; 1];
        v[0] = incoming;
        Ok(packet::to_string(&v)?)
    } else {
        // Long mode
        debug!("{}", "received long mode packet");
        ctx.write(&sid, &ACK)?;
        ctx.read(&sid, &mut incoming)?; // Length
        let (_, chunks) = packet::to_len_chunks(&incoming);
        let mut v = vec![[0; IO_SIZE]; chunks];
        for i in 0..chunks {
            ctx.write(&sid, &ACK)?;
            ctx.read(&sid, &mut v[i])?;
        }
        Ok(packet::to_string(&v)?)
    }
}

/// Usb protocol does not support transmitting the error code with the
/// response. So we peek at the response and parse it to see if it is an
/// {\"error\": code} object. If it is, we translate this from Ok to Err
/// to present sane behavior to caller.
pub fn translate_error(response: String) -> Result<String> {
    let packet = serde_json::from_str::<Value>(&response);
    let packet = if let Ok(p) = packet {
        p.as_object()
            .and_then(|object| {
                if object.keys().len() == 1
                    && object.contains_key("error")
                    && object["error"].is_number()
                {
                    Some(object["error"].as_i64())
                } else {
                    None
                }
            })
            .and_then(|x| match x {
                Some(200) => None,
                Some(400) => Some(ApiError::Linq400),
                Some(403) => Some(ApiError::Linq403),
                Some(404) => Some(ApiError::Linq404),
                Some(500) => Some(ApiError::Linq500),
                Some(504) => Some(ApiError::Linq504),
                _ => Some(ApiError::LinqUnknown),
            })
    } else {
        None
    };
    match packet {
        Some(e) => Err(e.into()),
        None => Ok(response),
    }
}

/// Drive some packets and make a request to a K64 USB device
pub fn request_raw(
    ctx: &impl ReaderWriter,
    sid: &str,
    r: Request,
) -> Result<String> {
    let mut retry: u8 = 0;
    let mut result: Result<String>;

    // This loop is designed to shovel our shit into the K64 no matter how much
    // it kicks and screams. Should this loop fail us, then either the K64 has
    // been unplugged, or we killed it!
    loop {
        result = make_request(ctx, sid, &r).and_then(|r| translate_error(r));
        debug!("{:.10?}", result);
        match &result {
            Ok(_) => break,
            Err(IoError::ApiError(ApiError::Linq504)) => {
                // TODO inject time delay
                retry = retry + 1;
            }
            Err(IoError::Usb(_)) => {
                // TODO inject time delay
                //      distinguish USB protocol error (needs flush) vs other,
                //      (no flush needed)
                retry = retry + 1;
                flush(ctx, sid);
            }
            Err(e) => {
                warn!("{}", e);
                break;
            }
        }
        if retry > MAX_RETRY {
            break;
        };
    }
    result
}

/// Return a driver handle for a K64 USB device
pub fn open<T: ReaderWriter>(
    ctx: &T,
    sid: &str,
) -> Result<(String, fn(ctx: &T, sid: &str, r: Request) -> Result<String>)> {
    info!("[{}] open", sid);
    let mut retry = 0;
    loop {
        let response: Result<AboutResponse> =
            request(ctx, &sid, Request::get("/ATX/about"));
        match response {
            Ok(a) => return Ok((a.about.sid, request_raw)),
            Err(e) => {
                retry = retry + 1;
                if retry > MAX_RETRY {
                    return Err(e);
                }
            }
        }
    }
}
