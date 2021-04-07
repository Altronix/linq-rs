use crate::error::*;
use crate::request::Request;

/// A  Devices implement there on forms of reading and writing
pub trait ReaderWriter: Writer + Reader {}

/// A writer trait used to stub our binding when testing
pub trait Writer {
    fn write<'a>(&self, s: &'a str, bytes: &[u8]) -> Result<usize>;
}

/// A read trait used to stub our binding when testing
pub trait Reader {
    fn read<'a>(&self, s: &'a str, bytes: &mut [u8]) -> Result<usize>;
}
