extern crate shiori_hglobal;
extern crate shiori_parser;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[cfg(any(windows))]
extern crate winapi;
#[cfg(any(windows))]
mod windows;
