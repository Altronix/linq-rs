use crate::error::*;
use crate::request::Request;
/// Supported transfer size of HID protocol
pub const IO_SIZE: usize = 64;

/// K64 start of message
pub const PREAMBLE: [u8; IO_SIZE] = [
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
];

/// An ACK packet
pub const ACK: [u8; IO_SIZE] = [
    b'A', b'C', b'K', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Struct for helping parse and manipulate HID packets in 64 byte chunks
pub type Packets = ([u8; IO_SIZE], Vec<[u8; IO_SIZE]>);

/// Take a length packet and convert into usable number
pub fn to_len(packet: &[u8; IO_SIZE]) -> usize {
    usize::from(packet[0]) + (usize::from(packet[1]) << 8)
}

/// Take a length packet and calculate how many chunks
pub fn to_len_chunks(packet: &[u8; IO_SIZE]) -> (usize, usize) {
    let size = to_len(packet);
    let chunks = if size % IO_SIZE == 0 {
        size / IO_SIZE
    } else {
        size / IO_SIZE + 1
    };
    (size, chunks)
}

/// Prepare a length for transmit in packet frame
pub fn from_len(l: usize) -> [u8; IO_SIZE] {
    let mut u: [u8; IO_SIZE] = [0; IO_SIZE];
    u[0] = l as u8;
    u[1] = (l / 256) as u8;
    u
}

/// Convert a request into packets for transfer
pub fn from_request(r: &Request) -> Packets {
    from_str(r.format_with_null_terminators().as_ref())
}

/// We have a string and want some packets
pub fn from_str(s: &str) -> Packets {
    let rlen = s.len();
    let len: [u8; IO_SIZE] = from_len(rlen);
    let n = if rlen % IO_SIZE == 0 {
        rlen / IO_SIZE
    } else {
        rlen / IO_SIZE + 1
    };
    let p: Vec<[u8; IO_SIZE]> = (0..n)
        .map(|i| {
            let mut v: [u8; IO_SIZE] = [0; IO_SIZE];
            let start = i * IO_SIZE;
            let c = if start + IO_SIZE > rlen {
                rlen - start
            } else {
                IO_SIZE
            };
            v[..c].copy_from_slice(&s[start..start + c].as_bytes());
            v
        })
        .collect();
    (len, p)
}

/// We collected a stream of packets into an array and we want them as packet
pub fn from_vec(v: Vec<[u8; IO_SIZE]>) -> Packets {
    // TODO prepend length instead of assuming it's attached
    let p = v[1..].iter().map(|x| x.to_owned()).collect();
    (v[0], p)
}

/// We have some packets and want a string
/// We unwrap here because we are asserting caller gives valid strings
pub fn to_string(p: &Vec<[u8; IO_SIZE]>) -> Result<String> {
    p[..]
        .into_iter()
        .map(|x| {
            std::str::from_utf8(x)
                .map_err(|_| IoError::Parser("bad utf8".to_string()))
                .and_then(|s| Ok(s.trim_matches(char::from(0))))
        })
        .collect()
}
