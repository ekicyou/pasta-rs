extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate smallvec;

pub mod di;
pub mod dic;
pub mod error;
pub mod parser;
pub mod ss;
pub mod ss_fmt;
