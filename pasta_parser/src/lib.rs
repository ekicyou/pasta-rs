extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

