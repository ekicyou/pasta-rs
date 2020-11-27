use std::iter::{IntoIterator, Iterator};
use std::marker::PhantomData;

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
pub struct Script {
    pub items: Vec<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attrs {
    pub items: Vec<AST>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Togaki {
    pub items: Vec<AST>,
}

//=============================================================
//

pub trait ItemsAST {
    fn items(&self) -> &Vec<AST>;
}
pub struct ItemsASTIntoIter<'a, T: ItemsAST> {
    items: std::slice::Iter<'a, AST>,
    phantom: PhantomData<fn() -> T>,
}
impl<'a, T: ItemsAST> ItemsASTIntoIter<'a, T> {
    fn new(items: &'a T) -> Self {
        let items = items.items().into_iter();
        Self {
            items,
            phantom: PhantomData,
        }
    }
}
impl<'a, T: ItemsAST> Iterator for ItemsASTIntoIter<'a, T> {
    type Item = &'a AST;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}

impl ItemsAST for Script {
    fn items(&self) -> &Vec<AST> {
        &self.items
    }
}
impl ItemsAST for Togaki {
    fn items(&self) -> &Vec<AST> {
        &self.items
    }
}
impl ItemsAST for Attrs {
    fn items(&self) -> &Vec<AST> {
        &self.items
    }
}

//=============================================================
//

pub trait ExprAST {
    fn expr(&self) -> &Box<AST>;
}
pub struct ExprASTIntoIter<'a, T: ExprAST> {
    expr: &'a T,
    none: bool,
}
impl<'a, T: ExprAST> ExprASTIntoIter<'a, T> {
    fn new(expr: &'a T) -> Self {
        Self { expr, none: false }
    }
}
impl<'a, T: ExprAST> Iterator for ExprASTIntoIter<'a, T> {
    type Item = &'a AST;
    fn next(&mut self) -> Option<Self::Item> {
        match self.none {
            true => None,
            _ => {
                self.none = true;
                Some(&(self.expr.expr()))
            }
        }
    }
}

impl ExprAST for Action {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for Require {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for Either {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for Forget {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for Memory {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for ShortJump {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}
impl ExprAST for LongJump {
    fn expr(&self) -> &Box<AST> {
        &self.expr
    }
}

//=============================================================
//

/// [code, err, comment] の順で有効な要素を返すイテレータを返す。
impl<'a> IntoIterator for &'a Line {
    type Item = &'a AST;
    type IntoIter = LineIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        LineIntoIter::new(self)
    }
}

pub struct LineIntoIter<'a> {
    flat: LineFlatIter<'a>,
}
impl<'a> LineIntoIter<'a> {
    fn new(line: &'a Line) -> Self {
        let flat = LineFlatIter { line, index: 0 };
        Self { flat }
    }
}
impl<'a> Iterator for LineIntoIter<'a> {
    type Item = &'a AST;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.flat.next() {
                None => {
                    return None;
                }
                Some(Some(a)) => {
                    return Some(a);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

pub struct LineFlatIter<'a> {
    line: &'a Line,
    index: i32,
}
impl<'a> Iterator for LineFlatIter<'a> {
    type Item = Option<&'a AST>;
    fn next(&mut self) -> Option<Self::Item> {
        let rc = match self.index {
            0 => &self.line.code,
            1 => &self.line.err,
            2 => &self.line.comment,
            _ => {
                return None;
            }
        };
        self.index += 1;
        let rc = match rc.as_ref() {
            Some(a) => Some(&**a),
            _ => None,
        };
        Some(rc)
    }
}

//=============================================================
//

/// [doc_comment, code, err, comment] を列挙するイテレータを返す
impl<'a> IntoIterator for &'a Script {
    type Item = &'a AST;
    type IntoIter = ScriptIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ScriptIntoIter::new(self)
    }
}

pub struct ScriptIntoIter<'a> {
    script: std::slice::Iter<'a, AST>,
    line: Option<LineIntoIter<'a>>,
}
impl<'a> ScriptIntoIter<'a> {
    fn new(script: &'a Script) -> Self {
        let script = (&script.items).into_iter();
        Self { script, line: None }
    }
    fn next_impl(&mut self) -> Option<&'a AST> {
        loop {
            if let Some(line) = &mut self.line {
                match line.next() {
                    Some(a) => {
                        return Some(a);
                    }
                    _ => (),
                }
            }
            return match self.script.next() {
                Some(&AST::Line(ref a)) => {
                    self.line = Some(a.into_iter());
                    continue;
                }
                Some(a) => Some(a),
                None => None,
            };
        }
    }
}
impl<'a> Iterator for ScriptIntoIter<'a> {
    type Item = &'a AST;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl()
    }
}
