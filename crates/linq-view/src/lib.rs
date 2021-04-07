#[cfg(test)]
mod tests;

pub mod error;
mod resource;
mod view;
mod window;
use error::*;

pub use resource::{Html, Js};
pub use window::*;
