#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    Unimplemented,

    DocComment(DocComment),
    Comment(Comment),
    Error(Error),

    Expr(Expr),

    Action(Action),
    Require(Require),
    Either(Either),
    Forget(Forget),
    Memory(Memory),
    Attrs(Attrs),

    Hasira(Hasira),
    Serif(Serif),
    Togaki(Togaki),
    ShortJump(ShortJump),
    LongJump(LongJump),
    Anchor(Anchor),

    Line(Line),
    Script(Script),
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
    pub name: String,
    pub attrs: Option<Box<AST>>,
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
