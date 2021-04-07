use serde::Serialize;
use std::borrow::Cow;

/// Useful strings for mapping enum to strings for transmitting
const GET: &'static str = "GET";
const POST: &'static str = "POST";
const DELETE: &'static str = "DELETE";

#[derive(Clone, PartialEq, Debug)]
pub enum Request {
    /// Get Request accepts a Path
    Get(String),
    /// Post Request accepts a Path, and Data string
    Post(String, String),
    /// Delete Request accepts a Path
    Delete(String),
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Request::Get(p) => write!(f, "[{}] {}", "GET", p),
            Request::Post(p, b) => write!(f, "[{}] {} {:.18}...", "POST", p, b),
            Request::Delete(p) => write!(f, "[{}] {}", "DELETE", p),
        }
    }
}

impl Request {
    /// Helper routine to extract typical arguments for building URL
    pub fn format_with_null_terminators(&self) -> String {
        match self {
            Request::Get(path) => format!("{}\0{}", GET, path),
            Request::Post(p, b) => format!("{}\0{}\0{}", POST, p, b),
            Request::Delete(path) => format!("{}\0{}", DELETE, path),
        }
    }

    /// Create a GET request
    pub fn get<'a>(path: &'a str) -> Self {
        Request::Get(path.to_owned())
    }

    /// Create a POST req where data can be serialized from a native rust object
    pub fn post<'a, T: Serialize>(path: &'a str, data: &T) -> Self {
        let t = serde_json::to_string(&data).unwrap(); // TODO return result
        Request::post_raw(path, t)
    }

    /// Create a POST request that cannot be serialized. (Useful for proxying)
    pub fn post_raw<'a, S: Into<Cow<'a, str>>>(path: &'a str, data: S) -> Self {
        let data: String = match data.into() {
            Cow::Borrowed(s) => s.to_owned(),
            Cow::Owned(s) => s,
        };
        Request::Post(path.to_owned(), data)
    }

    /// Create a DELETE request
    pub fn delete<'a>(serial: &'a str, path: &'a str) -> Self {
        Request::Delete(path.to_owned())
    }
}
