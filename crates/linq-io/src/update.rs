use super::request::Request;
use crate::error::*;
use linq_db::k64::{DashboardUpdate, DashboardUpdateImage};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct DashboardUpdatePackets(pub Vec<Request>, pub Vec<Request>);
impl DashboardUpdatePackets {
    /// Have file location, want requests
    pub fn parse_file(path: &str) -> Result<Self> {
        let mut file = File::open(Path::new(&path))?;
        let mut update = String::new();
        file.read_to_string(&mut update)?;
        Self::parse(&update)
    }

    /// We have a JSON string Dashboard Update and we want a vec of requests
    /// NOTE: we reverse our array because we pop() them off before tx and pop
    ///       takes off the end of the list...
    pub fn parse(u: &str) -> Result<Self> {
        let mut update = serde_json::from_str::<DashboardUpdate>(u)
            .map_err(|x| IoError::Parser(x.to_string()))?;
        if update.files.len() < 2 {
            return Err(IoError::Parser("bad update file".to_string()));
        }
        let website = Self::map_requests(update.files.pop().unwrap())?;
        let firmware = Self::map_requests(update.files.pop().unwrap())?;
        Ok(DashboardUpdatePackets(firmware, website))
    }

    /// Take an update image and return a Vector of requests
    fn map_requests(update: DashboardUpdateImage) -> Result<Vec<Request>> {
        update
            .update
            .into_iter()
            .map(|u| Ok(Request::post("/ATX/exe/update", &u)))
            .rev()
            .collect()
    }
}
