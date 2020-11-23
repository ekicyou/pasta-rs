#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    Unimplemented,

    DocComment(String),
    Error(usize, usize, char, String),
    Comment(String),

    Expr(String),

    Action(Box<AST>),
    Require(Box<AST>),
    Either(Box<AST>),
    Forget(Box<AST>),
    Memory(Box<AST>),
    Attrs(Vec<AST>),

    Hasira(usize, String, Option<Box<AST>>),

    Serif(String),
    Togaki(Vec<AST>),

    Line(Option<Box<AST>>, Option<Box<AST>>, Option<Box<AST>>),
    Script(Vec<AST>),
}
