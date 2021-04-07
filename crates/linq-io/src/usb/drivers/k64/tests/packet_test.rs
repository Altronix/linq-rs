use super::super::packet;
use crate::request::Request;

pub const TEST_DATA: &'static str = r#"
        {
          "about": {
            "siteId": "Site ID",
            "prjVersion": "2.6.6",
            "prjVersionRc": "",
            "productKey": "",
            "product": "LINQ2",
            "mqxVersion": "4.2.0",
            "atxVersion": "2.5.2",
            "atxVersionRc": "1",
            "sslVersion": "3.13.0",
            "webVersion": "2.0.0",
            "mfg": "Altronix",
            "user": "",
            "mac": "CC:67:AB:FF:28:A2",
            "sid": "f4q4riVN1GndwjSMmseFG-B_hUHrkze0oBUyKVyOzwg=",
            "iicAddr": 0,
            "policies": 0,
            "users": {},
            "address": 244,
            "io": 0
          }
        }"#;

#[test]
fn test_packet_to_string() {
    let p0: packet::Packets = packet::from_str(TEST_DATA);
    assert_eq!(packet::to_string(&p0.1).unwrap(), TEST_DATA);
}

#[test]
fn test_packet_from_len() {
    let len = packet::from_len(427);
    assert_eq!(len[0], 0xab);
    assert_eq!(len[1], 0x01);
}

#[test]
fn test_packet_from_request_get() {
    let request = Request::get("/ATX/about");
    let packets = packet::from_request(&request);
    assert_eq!(packets.1[0][0..3], [b'G', b'E', b'T']);
}

#[test]
fn test_packet_from_request_post() {
    let request = Request::post_raw("/ATX/about", "{\"foo\"}");
    let packets = packet::from_request(&request);
    assert_eq!(packets.1[0][0..4], [b'P', b'O', b'S', b'T']);
}

#[test]
fn test_packet_to_len_chunks() {
    let len = packet::from_len(427);
    let (size, chunks) = packet::to_len_chunks(&len);
    assert_eq!(size, 427);
    assert_eq!(chunks, 7);
}
