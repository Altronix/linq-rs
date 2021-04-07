use super::packet_test::*;
use crate::error::{ApiError, IoError, Result};
use crate::request::Request;
use crate::usb::drivers::driver::{Reader, ReaderWriter, Writer};
use crate::usb::drivers::k64;
use crate::usb::drivers::k64::packet::*;
use std::cell::RefCell;
use std::collections::VecDeque;

/// Any outgoing or incoming packets are simulated in here
pub struct MockPackets {
    pub incoming: RefCell<VecDeque<Result<[u8; IO_SIZE]>>>,
    pub outgoing: RefCell<Vec<[u8; IO_SIZE]>>,
}

impl MockPackets {
    pub fn new() -> Self {
        MockPackets {
            incoming: RefCell::new(VecDeque::new()),
            outgoing: RefCell::new(Vec::new()),
        }
    }

    fn add_incoming(&mut self, bytes: [u8; IO_SIZE]) {
        self.incoming.borrow_mut().push_back(Ok(bytes));
    }

    fn add_incoming_error(&mut self, e: IoError) {
        self.incoming.borrow_mut().push_back(Err(e));
    }
}

impl ReaderWriter for MockPackets {}

impl Writer for MockPackets {
    fn write<'a>(&self, _: &'a str, bytes: &[u8]) -> Result<usize> {
        let mut b: [u8; IO_SIZE] = [0; IO_SIZE];
        b.copy_from_slice(&bytes[..]);
        self.outgoing.borrow_mut().push(b);
        Ok(bytes.len())
    }
}

impl Reader for MockPackets {
    fn read<'a>(&self, _: &'a str, bytes: &mut [u8]) -> Result<usize> {
        let incoming = self
            .incoming
            .borrow_mut()
            .pop_front()
            .map_or(Err(IoError::Unknown), |v| v);
        match incoming {
            Ok(incoming) => {
                bytes.copy_from_slice(&incoming[..]);
                Ok(incoming.len())
            }
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test_long() {
    let mut mock = MockPackets::new();
    let (len, packets) = from_str(TEST_DATA);
    mock.add_incoming(ACK);
    mock.add_incoming(ACK);
    mock.add_incoming(PREAMBLE);
    mock.add_incoming(len);
    for i in packets {
        mock.add_incoming(i);
    }
    assert!(k64::request_raw(&mut mock, "", Request::get("")).is_ok());
}

#[test]
fn test_short() {
    let mut mock = MockPackets::new();
    let (_, packets) = from_str("{\"siteId\":\"foo\"}");
    mock.add_incoming(ACK);
    mock.add_incoming(ACK);
    mock.add_incoming(packets[0]);
    assert!(k64::request_raw(&mut mock, "", Request::get("")).is_ok());
}

#[test]
fn test_bad() {
    let mut mock = MockPackets::new();
    mock.add_incoming(ACK);
    mock.add_incoming(PREAMBLE); // <-- should be ACK!
    assert!(k64::request_raw(&mut mock, "", Request::get("")).is_err());
}

#[test]
fn test_translate_api_error() {
    let mut mock = MockPackets::new();
    let (_, packets) = from_str("{\"error\":400}");
    mock.add_incoming(ACK);
    mock.add_incoming(ACK);
    mock.add_incoming(packets[0]);
    let response = k64::request_raw(&mut mock, "", Request::get(""));
    let result = if let Err(IoError::ApiError(_)) = response {
        true
    } else {
        false
    };
    assert_eq!(result, true);
}

#[test]
fn test_flush() {
    let mut mock = MockPackets::new();
    let (_, packets) = from_str("{\"siteId\":\"foo\"}");
    mock.add_incoming(packets[0]); // Garbage in OS incoming buffer
    mock.add_incoming(packets[0]); // Flush it all out
    mock.add_incoming(packets[0]); // Flush it all out
    mock.add_incoming(packets[0]); // Flush it all out
    mock.add_incoming(packets[0]); // Flush it all out
    mock.add_incoming_error(IoError::Unknown); // No more to flush
    mock.add_incoming(ACK);
    mock.add_incoming(ACK);
    mock.add_incoming(packets[0]);
    assert!(k64::request_raw(&mut mock, "", Request::get("")).is_ok());
}
