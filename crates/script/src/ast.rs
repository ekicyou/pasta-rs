use std::iter::{IntoIterator, Iterator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    Unimplemented,

    Script(Script),
    Line(Line),

    DocComment(DocComment),
    Comment(Comment),
    Error(Error),

    Hasira(Hasira),
    Togaki(Togaki),

    Serif(Serif),

    Attrs(Attrs),

    Action(Action),
    Require(Require),
    Either(Either),
    Forget(Forget),
    Memory(Memory),
    LongJump(LongJump),
    ShortJump(ShortJump),
    Anchor(Anchor),

    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Script {
    pub items: Vec<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line {
    pub code: Option<Box<AST>>,
    pub err: Option<Box<AST>>,
    pub comment: Option<Box<AST>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    pub start: usize,
    pub end: usize,
    pub error_token: char,
    pub error_str: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hasira {
    pub level: usize,
    pub name: String,
    pub attrs: Option<Box<AST>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anchor {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocComment {
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Serif {
    pub serif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Require {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Either {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Forget {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Memory {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShortJump {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LongJump {
    pub expr: Box<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attrs {
    pub items: Vec<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Togaki {
    pub items: Vec<AST>,
}

impl<'a> IntoIterator for &'a Script {
    type Item = &'a AST;
    type IntoIter = ScriptIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ScriptIntoIter::new(self)
    }
}

pub struct ScriptIntoIter<'a> {
    script: std::slice::Iter<'a, AST>,
}
impl<'a> ScriptIntoIter<'a> {
    fn new(script: &'a Script) -> Self {
        let script = (&script.items).into_iter();
        ScriptIntoIter { script }
    }
}
impl<'a> Iterator for ScriptIntoIter<'a> {
    type Item = &'a AST;
    fn next(&mut self) -> Option<Self::Item> {
        self.script.next()
    }
}
