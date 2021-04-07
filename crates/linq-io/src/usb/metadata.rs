use super::binding::Driver;
use serde::{Deserialize, Serialize};

/// Helper for reading summary
#[derive(Debug, Deserialize, Serialize)]
pub struct Summary {
    pub vendor: u32,
    pub product: u32,
    pub serial: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsbMetadata {
    pub vid: u32,
    pub pid: u32,
    pub sid: String,
    pub serial: String,
    #[serde(skip)]
    pub driver: Driver,
}

impl UsbMetadata {
    pub fn new(serial: &str, driver: Driver, s: &Summary) -> Self {
        let (vid, pid, sid) = (s.vendor, s.product, s.serial.to_owned());
        UsbMetadata {
            vid,
            pid,
            sid,
            serial: serial.to_owned(),
            driver,
        }
    }
}
