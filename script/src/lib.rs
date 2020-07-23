extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Position;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

/// Parses a `&str` starting from `rule`.
pub fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    use pest::Parser;
    PastaParser::parse(rule, input)
}

/// Parses a `&str` starting from `rule`.
pub fn parse_nth(rule: Rule, n: usize, input: &str) -> Result<Pair<Rule>, Error<Rule>> {
    let item = parse(rule, input)?.flatten().nth(n);
    match item {
        Some(a) => Ok(a),
        _ => {
            let variant = ErrorVariant::CustomError {
                message: String::from("item not found"),
            };
            let pos = Position::from_start(input);
            let err = Error::new_from_pos(variant, pos);
            Err(err)
        }
    }
}
