#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate libc;

#[cfg(any(windows))]
#[macro_use]
extern crate winapi;

#[cfg(any(windows))]
mod win32;

mod shiori3;
