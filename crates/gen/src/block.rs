use pasta_script::ast::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootBlock {
    pub doc_comment: Option<DocComment>,
    pub hasira: Vec<HasiraBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub id: String,
    pub either: Vec<Either>,
    pub action: Vec<Action>,
    pub memory: Vec<Memory>,
    pub forget: Vec<Forget>,
    pub error: Option<Error>,
    pub comment: Option<Comment>,
}
impl Attribute {
    fn new(name: String, id: String) -> Self {
        Self {
            name: name,
            id: id,
            either: Vec::new(),
            action: Vec::new(),
            memory: Vec::new(),
            forget: Vec::new(),
            error: None,
            comment: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HasiraBlock {
    pub attr: Attribute,
    pub items: Vec<AnchorBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnchorBlock {
    pub attr: Attribute,
    pub items: Vec<ActorBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorBlock {
    pub attr: Attribute,
    pub items: Vec<SerifBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerifBlock {
    pub items: Vec<SerifItem>,
}

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
    Comment(Comment),
    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Builder {
    now_level: i32,
    l1: RootBlock,
    l2: Option<HasiraBlock>,
    l3: Option<AnchorBlock>,
    l4: Option<ActorBlock>,
    l5: Option<SerifBlock>,
}

impl Builder {
    fn new() -> Self {
        Self {
            now_level: 1,
            l1: RootBlock {
                doc_comment: None,
                hasira: Vec::new(),
            },
            l2: None,
            l3: None,
            l4: None,
            l5: None,
        }
    }
    fn root(self) -> RootBlock {
        self.l1
    }

    /// l2 -> commit
    fn commit2(&mut self) {
        self.commit3();
        let item = self.l2.take();
        if let Some(item) = item {
            self.l1.hasira.push(item)
        }
    }
    /// l3 -> commit
    fn commit3(&mut self) {
        self.commit4();
        let item = self.l3.take();
        let own = &mut self.l2;
        if let Some(item) = item {
            if let Some(own) = own.as_mut() {
                own.items.push(item);
            }
        }
    }
    /// l4 -> commit
    fn commit4(&mut self) {
        self.commit5();
        let item = self.l4.take();
        let own = &mut self.l3;
        if let Some(item) = item {
            if let Some(own) = own.as_mut() {
                own.items.push(item);
            }
        }
    }
    /// l5 -> commit
    fn commit5(&mut self) {
        let item = self.l5.take();
        let own = &mut self.l4;
        if let Some(item) = item {
            if let Some(own) = own.as_mut() {
                own.items.push(item);
            }
        }
    }

    /// l3 push
    fn push_anchor(&mut self, ast: &AST) {
        self.commit3();
        let l3 = match ast.clone() {
            AST::Anchor(a) => {
                let name = match &*a.expr {
                    AST::Expr(expr) => &expr.expr,
                    AST::ExprOrNum(expr) => &expr.expr,
                    _ => panic!(),
                };
                AnchorBlock {
                    attr: Attribute::new(name.to_owned(), format!("L{}", name)),
                    items: Vec::new(),
                }
            }
            _ => panic!(),
        };
        self.l3 = Some(l3);
    }

    /// l4 push
    fn push_actor(&mut self, ast: &AST) {
        self.commit4();
        let l4 = match ast.clone() {
            AST::Hasira(h) if h.level == 0 => ActorBlock {
                attr: Attribute::new(h.name.to_owned(), format!("A{}", h.name)),
                items: Vec::new(),
            },
            _ => panic!(),
        };
        self.l4 = Some(l4);
    }
    /// l5 push
    fn push_jump(&mut self, ast: &AST) {
        self.commit5();
        let mut l5 = SerifBlock { items: Vec::new() };
        let item = match ast.clone() {
            AST::LongJump(a) => SerifItem::LongJump(a),
            AST::ShortJump(a) => SerifItem::ShortJump(a),
            _ => panic!(),
        };
        l5.items.push(item);
        self.l5 = Some(l5);
    }
    /// l5 push
    fn push_hasira(&mut self, togaki: &Togaki) {
        self.commit5();
        let mut l5 = SerifBlock { items: Vec::new() };
        for ast in &togaki.items {
            let item = match ast.clone() {
                AST::Action(a) => SerifItem::Action(a),
                AST::Require(a) => SerifItem::Require(a),
                AST::Either(a) => SerifItem::Either(a),
                AST::Forget(a) => SerifItem::Forget(a),
                AST::Memory(a) => SerifItem::Memory(a),
                AST::LongJump(a) => SerifItem::LongJump(a),
                AST::ShortJump(a) => SerifItem::ShortJump(a),
                AST::Serif(a) => SerifItem::Serif(a),
                AST::Comment(a) => SerifItem::Comment(a),
                AST::Error(a) => SerifItem::Error(a),
                _ => panic!(),
            };
            l5.items.push(item)
        }
        self.l5 = Some(l5);
    }
}

pub fn scan(script: &Script) -> RootBlock {
    let mut l1 = RootBlock {
        doc_comment: None,
        hasira: Vec::new(),
    };

    l1
}
