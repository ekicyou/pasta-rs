use crate::ast;
use crate::ast::*;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Position;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "pasta.pest"]
pub struct PastaParser;

pub type GrammarError = pest::error::Error<Rule>;
pub type GrammarResult<T> = std::result::Result<T, GrammarError>;

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
pub type ParserResult<T> = std::result::Result<T, ParserError>;
pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;
pub type Nodes<'i> = pest_consume::Nodes<'i, Rule, ()>;

#[allow(dead_code)]
static BUG: &str = "実装バグがあります。";

// This is the other half of the parser, using pest_consume.
#[pest_consume::parser]
impl PastaParser {
    pub fn EOI(_input: Node) -> ParserResult<()> {
        Ok(())
    }

    pub fn doc_comment(n: Node) -> ParserResult<AST> {
        Ok(AST::DocComment(DocComment {
            comment: n.as_str().to_owned(),
        }))
    }

    pub fn error(n: Node) -> ParserResult<AST> {
        let span = n.as_span();
        let start = span.start();
        let end = span.end();
        let error_str = n.as_str().to_owned();
        let mut items = n.children();
        let m = items.next().ok_or(n.error(BUG))?;
        let error_token = m.as_str().chars().next().ok_or(n.error(BUG))?;
        Ok(AST::Error(ast::Error {
            start,
            end,
            error_token,
            error_str,
        }))
    }

    pub fn comment(n: Node) -> ParserResult<AST> {
        Ok(AST::Comment(Comment {
            comment: n.as_str().to_owned(),
        }))
    }

    pub fn err_or_comment(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [comment(a)]=> a,
            [error(a)]=> a,
        ))
    }

    pub fn expr(n: Node) -> ParserResult<AST> {
        let mut items = n.children();
        let m = items.next().ok_or(n.error(BUG))?;
        let keyword = m.as_str().to_owned();
        Ok(AST::Expr(Expr { expr: keyword }))
    }

    pub fn action(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Action(Action{expr: Box::new(a)}),
        ))
    }

    pub fn require(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Require(Require{expr: Box::new(a)}),
        ))
    }

    pub fn either(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Either(Either{expr: Box::new(a)}),
        ))
    }

    pub fn forget(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Forget(Forget{expr: Box::new(a)}),
        ))
    }

    pub fn memory(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Memory(Memory{expr: Box::new(a)}),
        ))
    }

    pub fn long_jump(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::LongJump(LongJump{expr: Box::new(a)}),
        ))
    }

    pub fn short_jump(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::ShortJump(ShortJump{expr: Box::new(a)}),
        ))
    }

    pub fn anchor(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [expr(a)]=> AST::Anchor(Anchor{expr: Box::new(a)}),
        ))
    }

    pub fn h_attrs(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [h_attr(a)..]=>AST::Attrs(Attrs{items: a.collect()}),
        ))
    }

    pub fn h_attr(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [require(a)]=> a,
            [either(a)]=> a,
            [forget(a)]=> a,
            [memory(a)]=> a,
            [action(a)]=> a,
        ))
    }

    pub fn hasira_level(n: Node) -> ParserResult<usize> {
        let count = n.as_str().chars().count();
        Ok(count)
    }

    pub fn hasira_title(n: Node) -> ParserResult<&str> {
        Ok(n.as_str())
    }

    pub fn hasira_header(n: Node) -> ParserResult<(usize, &str)> {
        Ok(match_nodes!(n.into_children();
            [hasira_level(l),hasira_title(s)] => (l,s),
            [hasira_level(l)] => (l,""),
        ))
    }

    pub fn actor(n: Node) -> ParserResult<&str> {
        Ok(n.as_str())
    }

    pub fn actor_header(n: Node) -> ParserResult<&str> {
        Ok(match_nodes!(n.children();
            [actor(a)] => a,
        ))
    }

    pub fn hasira(n: Node) -> ParserResult<AST> {
        let (level, name, attrs) = match_nodes!(n.children();
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

        Ok(AST::Hasira(Hasira {
            level,
            name: name.to_owned(),
            attrs,
        }))
    }

    pub fn s_normal(n: Node) -> ParserResult<&str> {
        Ok(n.as_str())
    }

    pub fn escape(n: Node) -> ParserResult<char> {
        let m = n.children().next().ok_or(n.error(BUG))?;
        let c = m.as_str().chars().next().ok_or(n.error(BUG))?;
        Ok(c)
    }

    pub fn s_token(n: Node) -> ParserResult<(Option<&str>, Option<char>)> {
        Ok(match_nodes!(n.into_children();
            [s_normal(s)] => (Some(s), None),
            [escape(c)] => (None, Some(c)),
        ))
    }

    pub fn serif(n: Node) -> ParserResult<AST> {
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
                AST::Serif(Serif{serif: buf})
            },
        ))
    }

    pub fn t_attr(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [require(a)]=> a,
            [either(a)]=> a,
            [forget(a)]=> a,
            [memory(a)]=> a,
            [action(a)]=> a,
        ))
    }

    pub fn t_jump(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.children();
            [long_jump(a)] => a,
            [short_jump(a)] => a,
            [anchor(a)] => a,
        ))
    }
    pub fn t_item(n: Node) -> ParserResult<AST> {
        Ok(match_nodes!(n.into_children();
            [t_attr(a)]=> a,
            [serif(a)]=> a,
        ))
    }

    pub fn togaki(n: Node) -> ParserResult<AST> {
        Ok(AST::Togaki(Togaki {
            items: match_nodes!(n.into_children();
                [t_jump(a)]=> vec!{a},
                [t_item(a)..]=> a.collect(),
            ),
        }))
    }

    pub fn line(n: Node) -> ParserResult<AST> {
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
                        AST::Error(..) => err = Some(ast),
                        AST::Comment(..) => comment = Some(ast),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let code = code.map(|x| Box::new(x));
        let err = err.map(|x| Box::new(x));
        let comment = comment.map(|x| Box::new(x));
        Ok(AST::Line(Line { code, err, comment }))
    }

    pub fn script(n: Node) -> ParserResult<AST> {
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
        Ok(AST::Script(Script { items: vv }))
    }
}

pub fn parse_node<'i>(rule: Rule, input_str: &'i str) -> ParserResult<Nodes<'i>> {
    use pest_consume::Parser;
    PastaParser::parse(rule, input_str)
}

pub fn parse_one<'i>(rule: Rule, input_str: &'i str) -> ParserResult<Node<'i>> {
    parse_node(rule, input_str)?.single()
}
