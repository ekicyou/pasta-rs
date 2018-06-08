extern crate pasta_di;
extern crate shiori_hglobal;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate nom;
extern crate regex;

mod enums;
mod parsers;
mod req;
mod res;
mod recipe;
pub mod api;
