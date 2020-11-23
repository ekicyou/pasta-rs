#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    Unimplemented,

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
