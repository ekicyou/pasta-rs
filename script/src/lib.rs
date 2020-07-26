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

use pest_consume::match_nodes;
pub type ParserError = pest_consume::Error<Rule>;
pub type Result<T> = std::result::Result<T, pest_consume::Error<Rule>>;
pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;
pub type Nodes<'i> = pest_consume::Nodes<'i, Rule, ()>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    not_implement,

    doc_comment(String),
    error(usize, usize, char, String),
    comment(String),

    expr(String),

    action(Box<AST>),
    require(Box<AST>),
    either(Box<AST>),
    forget(Box<AST>),
    memory(Box<AST>),
    attrs(Vec<AST>),

    hasira(usize, String, Option<Box<AST>>),

    serif(String),
    togaki(Vec<AST>),

    line(Option<Box<AST>>, Option<Box<AST>>, Option<Box<AST>>),
    script(Vec<AST>),
}

#[allow(dead_code)]
static BUG: &str = "実装バグがあります。";

// This is the other half of the parser, using pest_consume.
#[pest_consume::parser]
impl PastaParser {
    pub fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    pub fn doc_comment(n: Node) -> Result<AST> {
        Ok(AST::doc_comment(n.as_str().to_owned()))
    }

    pub fn error(n: Node) -> Result<AST> {
        let span = n.as_span();
        let start = span.start();
        let end = span.end();
        let error_str = n.as_str().to_owned();
        let mut items = n.children();
        let m = items.next().ok_or(n.error(BUG))?;
        let error_token = m.as_str().chars().next().ok_or(n.error(BUG))?;
        Ok(AST::error(start, end, error_token, error_str))
    }

    pub fn comment(n: Node) -> Result<AST> {
        Ok(AST::comment(n.as_str().to_owned()))
    }

    pub fn err_or_comment(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [comment(a)]=> a,
            [error(a)]=> a,
        ))
    }

    pub fn expr(n: Node) -> Result<AST> {
        let mut items = n.children();
        let m = items.next().ok_or(n.error(BUG))?;
        let keyword = m.as_str().to_owned();
        Ok(AST::expr(keyword))
    }

    pub fn action(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::action(Box::new(a)),
        ))
    }

    pub fn require(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::require(Box::new(a)),
        ))
    }

    pub fn either(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::either(Box::new(a)),
        ))
    }

    pub fn forget(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::forget(Box::new(a)),
        ))
    }

    pub fn memory(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::memory(Box::new(a)),
        ))
    }

    pub fn h_attrs(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [h_attr(a)..]=>AST::attrs(a.collect()),
        ))
    }

    pub fn h_attr(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [require(a)]=> a,
            [either(a)]=> a,
            [forget(a)]=> a,
            [memory(a)]=> a,
            [action(a)]=> a,
        ))
    }

    pub fn hasira_level(n: Node) -> Result<usize> {
        let count = n.as_str().chars().count();
        Ok(count)
    }

    pub fn hasira_title(n: Node) -> Result<&str> {
        Ok(n.as_str())
    }

    pub fn hasira_header(n: Node) -> Result<(usize, &str)> {
        Ok(match_nodes!(n.into_children();
            [hasira_level(l),hasira_title(s)] => (l,s),
            [hasira_level(l)] => (l,""),
        ))
    }
    pub fn actor(n: Node) -> Result<&str> {
        Ok(n.as_str())
    }

    pub fn actor_header(n: Node) -> Result<&str> {
        Ok(match_nodes!(n.children();
            [actor(a)] => a,
        ))
    }

    pub fn hasira(n: Node) -> Result<AST> {
        let (level, title, attrs) = match_nodes!(n.children();
            [hasira_header(a),h_attrs(attrs)] => {
                let (l,s)=a;
                (l,s,Some(Box::new(attrs)))
            },
            [actor_header(a),h_attrs(attrs)] => (0,a,Some(Box::new(attrs))),
            [hasira_header(a)] => {
                let (l,s)=a;
                (l,s,None)
            },
            [actor_header(a)] => (0,a,None),
        );

        Ok(AST::hasira(level, title.to_owned(), attrs))
    }

    pub fn s_normal(n: Node) -> Result<&str> {
        Ok(n.as_str())
    }

    pub fn escape(n: Node) -> Result<char> {
        let m = n.children().next().ok_or(n.error(BUG))?;
        let c = m.as_str().chars().next().ok_or(n.error(BUG))?;
        Ok(c)
    }

    pub fn s_token(n: Node) -> Result<(Option<&str>, Option<char>)> {
        Ok(match_nodes!(n.into_children();
            [s_normal(s)] => (Some(s), None),
            [escape(c)] => (None, Some(c)),
        ))
    }

    pub fn serif(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [s_token(tokens)..] => {
                let mut buf = String::new();
                for t in tokens{
                    if let (Some(s),_) = t {
                        buf.push_str(s);
                    }
                    if let (_,Some(c)) = t {
                        buf.push(c);
                    }
                }
                AST::serif(buf)
            },
        ))
    }

    pub fn t_attr(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [require(a)]=> a,
            [either(a)]=> a,
            [forget(a)]=> a,
            [memory(a)]=> a,
            [action(a)]=> a,
        ))
    }

    pub fn t_item(n: Node) -> Result<AST> {
        Ok(match_nodes!(n.into_children();
            [t_attr(a)]=> a,
            [serif(a)]=> a,
        ))
    }

    pub fn togaki(n: Node) -> Result<AST> {
        Ok(AST::togaki(match_nodes!(n.into_children();
            [t_item(a)..]=> a.collect(),
        )))
    }

    pub fn line(n: Node) -> Result<AST> {
        let mut code = None;
        let mut err = None;
        let mut comment = None;
        for n in n.children() {
            match n.as_rule() {
                Rule::togaki => code = Some(Self::togaki(n)?),
                Rule::hasira => code = Some(Self::hasira(n)?),
                Rule::err_or_comment => {
                    let ast = Self::err_or_comment(n)?;
                    match ast {
                        AST::error(..) => err = Some(ast),
                        AST::comment(..) => comment = Some(ast),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let code = code.map(|x| Box::new(x));
        let err = err.map(|x| Box::new(x));
        let comment = comment.map(|x| Box::new(x));
        Ok(AST::line(code, err, comment))
    }

    pub fn script(n: Node) -> Result<AST> {
        let mut vv = Vec::new();
        for n in n.children() {
            let ast = match n.as_rule() {
                Rule::doc_comment => Self::doc_comment(n)?,
                Rule::line => Self::line(n)?,
                _ => {
                    continue;
                }
            };
            vv.push(ast);
        }
        Ok(AST::script(vv))
    }
}

pub fn parse_node<'i>(rule: Rule, input_str: &'i str) -> Result<Nodes<'i>> {
    use pest_consume::Parser;
    PastaParser::parse(rule, input_str)
}

pub fn parse_one<'i>(rule: Rule, input_str: &'i str) -> Result<Node<'i>> {
    parse_node(rule, input_str)?.single()
}
