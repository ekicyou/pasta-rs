#![cfg(windows)]
#![no_std]

extern crate winapi;

mod com_rc;

pub use self::com_rc::*;
