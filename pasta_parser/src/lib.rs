extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

/// Parses a `&str` starting from `rule`.
pub fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    use pest::Parser;
    PastaParser::parse(rule, input)
}
