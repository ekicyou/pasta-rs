extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
#[cfg(any(windows))]
extern crate winapi;

pub extern crate shiori_hglobal as hglobal;
pub extern crate shiori_parser as parser;

mod api;
pub mod error;
#[cfg(any(windows))]
mod windows;

pub use api::Shiori3;
#[cfg(any(windows))]
pub use windows::RawAPI;
