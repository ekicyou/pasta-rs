use pasta_script::ast::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootBlock {
    pub doc_comment: DocComment,
    pub hasira: Vec<HasiraBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub require: Vec<Require>,
    pub either: Vec<Either>,
    pub action: Vec<Action>,
    pub memory: Vec<Memory>,
    pub forget: Vec<Forget>,
    pub error: Option<Error>,
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HasiraBlock {
    pub attr: Attribute,
    pub items: Vec<ActorBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorBlock {
    pub attr: Attribute,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerifBlock {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SerifItem {
    Action(Action),
    Require(Require),
    Either(Either),
    Forget(Forget),
    Memory(Memory),
    LongJump(LongJump),
    ShortJump(ShortJump),
    Serif(Serif),
}
