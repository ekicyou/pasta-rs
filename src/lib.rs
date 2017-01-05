// common モジュール登録

#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate libc;

extern crate local_encoding;

mod shiori3;

// windows モジュール登録

#[cfg(any(windows))]
#[macro_use]
extern crate winapi;

#[cfg(any(windows))]
#[macro_use]
extern crate kernel32;


#[cfg(any(windows))]
mod gstr;

//#[cfg(any(windows))]
//#mod windows;

//#[cfg(any(windows))]
//#pub use windows::{DllMain, load, unload, request};
