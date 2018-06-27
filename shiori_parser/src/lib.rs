#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;

#[macro_use]
extern crate pest_derive;
extern crate pest;

mod shiori;

pub use shiori::ShioriParser;