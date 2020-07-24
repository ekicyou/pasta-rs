extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Position;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

type GrammarError = pest::error::Error<Rule>;
type GrammarResult<T> = std::result::Result<T, GrammarError>;

/// Parses a `&str` starting from `rule`.
pub fn parse(rule: Rule, input: &str) -> GrammarResult<Pairs<Rule>> {
    use pest::Parser;
    PastaParser::parse(rule, input)
}

/// Parses a `&str` starting from `rule`.
pub fn parse_nth(rule: Rule, n: usize, input: &str) -> GrammarResult<Pair<Rule>> {
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

pub type ParserError = pest_consume::Error<Rule>;
pub type Result<T> = std::result::Result<T, pest_consume::Error<Rule>>;
pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;
pub type Nodes<'i> = pest_consume::Nodes<'i, Rule, ()>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST<'i> {
    escape(char),
    comment(&'i str),
}

#[allow(dead_code)]
static BUG: &str = "実装バグがあります。";

// This is the other half of the parser, using pest_consume.
#[pest_consume::parser]
impl PastaParser {
    pub fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    pub fn escape<'i>(n: Node<'i>) -> Result<AST<'i>> {
        let m = n.children().next().ok_or(n.error(BUG))?;
        let c = m.as_str().chars().next().ok_or(n.error(BUG))?;
        Ok(AST::escape(c))
    }

    pub fn comment<'i>(n: Node<'i>) -> Result<AST<'i>> {
        Ok(AST::comment(n.as_str()))
    }
}

pub fn parse_node<'i>(rule: Rule, input_str: &'i str) -> Result<Nodes<'i>> {
    use pest_consume::Parser;
    PastaParser::parse(rule, input_str)
}
