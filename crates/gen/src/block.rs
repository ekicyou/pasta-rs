use pasta_script::ast::*;
use std::collections::HashMap;
use std::mem;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootBlock {
    pub doc_comment: Option<DocComment>,
    pub hasira: Vec<HasiraBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub id: String,
    pub require: Vec<Require>,
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
            require: Vec::new(),
            either: Vec::new(),
            action: Vec::new(),
            memory: Vec::new(),
            forget: Vec::new(),
            error: None,
            comment: None,
        }
    }
}
impl Attribute {
    fn id_prefix(&mut self, text: &str) {
        if text.len() > 0 {
            self.id = format!("{}_{}", text, &self.id);
        }
    }
    fn id_suffix(&mut self, text: &str) {
        if text.len() > 0 {
            self.id = format!("{}_{}", &self.id, text);
        }
    }
    fn set_id<S: Into<String>>(&mut self, id: S) {
        self.id = id.into();
    }
    fn id_num(&mut self, prefix: &str, i: u32) {
        self.id_prefix(prefix);
        let suffix = match i {
            0 => {
                return;
            }
            _ => format!("{}", i),
        };
        self.id_suffix(&suffix);
    }
}

pub fn fix_id<A: AttributeBlock>(prefix: &str, items: &mut Vec<A>) {
    let mut i = 0;
    for item in items {
        let attr = item.attr_mut();
        i += 1;
        let id = format!("{}{}", prefix, i);
        attr.set_id(id);
    }
}

unsafe fn to_mut_ref<T>(src: &T) -> &mut T {
    let src = (src) as *const T as *mut T;
    let src = &mut *src;
    src
}

pub trait AttributeBlock {
    fn attr(&self) -> &Attribute;
    fn attr_mut(&mut self) -> &mut Attribute;
}
impl AttributeBlock for HasiraBlock {
    fn attr(&self) -> &Attribute {
        &self.attr
    }
    fn attr_mut(&mut self) -> &mut Attribute {
        &mut self.attr
    }
}
impl AttributeBlock for AnchorBlock {
    fn attr(&self) -> &Attribute {
        &self.attr
    }
    fn attr_mut(&mut self) -> &mut Attribute {
        &mut self.attr
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
    pub items: Vec<SerifBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerifBlock {
    pub items: Vec<SerifItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SerifItem {
    Actor(Actor),
    Serif(Serif),
    Action(Action),
    Require(Require),
    Either(Either),
    Forget(Forget),
    Memory(Memory),
    LongJump(LongJump),
    ShortJump(ShortJump),
    Comment(Comment),
    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Builder {
    now_level: i32,
    l1: RootBlock,
    l2: Option<HasiraBlock>,
    l3: Option<AnchorBlock>,
    l4: Option<SerifBlock>,
    hv: [Option<Hasira>; 4],
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
            hv: [None, None, None, None],
        }
    }
    fn root(mut self) -> RootBlock {
        self = self.commit2();
        self.l1
    }

    /// l2 -> commit
    fn commit2(mut self) -> Self {
        self = self.commit3();
        let item = self.l2.take();
        if let Some(item) = item {
            self.l1.hasira.push(item)
        }
        self
    }
    /// l3 -> commit
    fn commit3(mut self) -> Self {
        self = self.commit4();
        let item = self.l3.take();
        let own = &mut self.l2;
        if let Some(item) = item {
            if let Some(own) = own.as_mut() {
                own.items.push(item);
            }
        }
        self
    }
    /// l4 -> commit
    fn commit4(mut self) -> Self {
        self = self.push_no_anchor();
        let item = self.l4.take();
        let own = &mut self.l3;
        if let Some(item) = item {
            if let Some(own) = own.as_mut() {
                own.items.push(item);
            }
        }
        self
    }

    /// アンカーが存在しない場合にブランクアンカーを追加する
    fn push_no_anchor(mut self) -> Self {
        if self.l3.is_none() {
            let name = "";
            let l3 = AnchorBlock {
                attr: Attribute::new(name.to_owned(), format!("L{}", name)),
                items: Vec::new(),
            };
            self.l3 = Some(l3);
        }
        self
    }

    fn push_error(mut self, ast: &AST) -> Self {
        match ast.clone() {
            AST::Error(a) => {
                if let Some(target) = self.l4.as_mut() {
                    target.items.push(SerifItem::Error(a));
                } else if let Some(target) = self.l3.as_mut() {
                    target.attr.error = Some(a);
                } else if let Some(target) = self.l2.as_mut() {
                    target.attr.error = Some(a);
                }
            }
            _ => panic!("push_error: ast={:?}", ast),
        };
        self
    }

    fn push_comment(mut self, ast: &AST) -> Self {
        match ast.clone() {
            AST::Comment(a) => {
                if let Some(target) = self.l4.as_mut() {
                    target.items.push(SerifItem::Comment(a));
                } else if let Some(target) = self.l3.as_mut() {
                    target.attr.comment = Some(a);
                } else if let Some(target) = self.l2.as_mut() {
                    target.attr.comment = Some(a);
                }
            }
            _ => panic!("push_comment: ast={:?}", ast),
        };
        self
    }

    /// l1 push
    fn push_doc_comment(mut self, ast: &AST) -> Self {
        match ast.clone() {
            AST::DocComment(a) => self.l1.doc_comment = Some(a),
            _ => panic!("push_doc_comment: ast={:?}", ast),
        };
        self
    }

    /// l2 push
    fn push_hasira(mut self, ast: &AST) -> Self {
        self = self.commit2();
        match ast.clone() {
            AST::Hasira(h) if h.level >= 4 => {
                self.hv[0] = Some(h);
                self.hv[1] = None;
                self.hv[2] = None;
            }
            AST::Hasira(h) if h.level == 3 => {
                self.hv[1] = Some(h);
                self.hv[2] = None;
            }
            AST::Hasira(h) if h.level == 2 => {
                self.hv[2] = Some(h);
            }
            AST::Hasira(h) if h.level == 1 => {
                let name = h.name.to_owned();
                let id = format!("H{}", &name);
                self.hv[3] = Some(h);
                let mut l2 = HasiraBlock {
                    attr: Attribute::new(name, id),
                    items: Vec::new(),
                };
                for h in &self.hv {
                    if let Some(h) = h.as_ref() {
                        if let Some(attrs) = &h.attrs {
                            if let AST::Attrs(attrs) = &**attrs {
                                for ast in &attrs.items {
                                    match ast.clone() {
                                        AST::Require(a) => l2.attr.require.push(a),
                                        AST::Either(a) => l2.attr.either.push(a),
                                        AST::Action(a) => l2.attr.action.push(a),
                                        AST::Memory(a) => l2.attr.memory.push(a),
                                        AST::Forget(a) => l2.attr.forget.push(a),
                                        _ => panic!("push_hasira: ast={:?}", ast),
                                    }
                                }
                            }
                        }
                    }
                }
                self.l2 = Some(l2);
            }
            _ => panic!("push_hasira: ast={:?}", ast),
        };
        self
    }

    /// l3 push
    fn push_anchor(mut self, ast: &AST) -> Self {
        self = self.commit3();
        let l3 = match ast.clone() {
            AST::Anchor(a) => {
                let name = match &*a.expr {
                    AST::Expr(expr) => &expr.expr,
                    AST::ExprOrNum(expr) => &expr.expr,
                    _ => panic!("push_anchor: ast={:?}", ast),
                };
                AnchorBlock {
                    attr: Attribute::new(name.to_owned(), format!("L{}", name)),
                    items: Vec::new(),
                }
            }
            _ => panic!("push_anchor: ast={:?}", ast),
        };
        self.l3 = Some(l3);
        self
    }

    /// l4 push
    fn push_actor(mut self, hasira: &Hasira) -> Self {
        self = self.commit4();
        let mut l4 = SerifBlock { items: Vec::new() };
        let actor = Actor {
            name: hasira.name.to_owned(),
        };
        let actor = SerifItem::Actor(actor);
        l4.items.push(actor);
        if let Some(attrs) = &hasira.attrs {
            if let AST::Attrs(attrs) = &**attrs {
                for ast in &attrs.items {
                    let item = match ast.clone() {
                        AST::Action(a) => SerifItem::Action(a),
                        AST::Require(a) => SerifItem::Require(a),
                        AST::Either(a) => SerifItem::Either(a),
                        AST::Forget(a) => SerifItem::Forget(a),
                        AST::Memory(a) => SerifItem::Memory(a),
                        AST::Serif(a) => SerifItem::Serif(a),
                        AST::Comment(a) => SerifItem::Comment(a),
                        AST::Error(a) => SerifItem::Error(a),
                        _ => panic!("push_serif: ast={:?}", ast),
                    };
                    l4.items.push(item);
                }
            }
        }
        self.l4 = Some(l4);
        self
    }
    /// l4 push
    fn push_jump(mut self, ast: &AST) -> Self {
        self = self.commit4();
        let mut l4 = SerifBlock { items: Vec::new() };
        let item = match ast.clone() {
            AST::LongJump(a) => SerifItem::LongJump(a),
            AST::ShortJump(a) => SerifItem::ShortJump(a),
            _ => panic!("push_jump: ast={:?}", ast),
        };
        l4.items.push(item);
        self.l4 = Some(l4);
        self
    }
    /// l4 push
    fn push_serif(mut self, togaki: &Togaki) -> Self {
        self = self.commit4();
        let mut l4 = SerifBlock { items: Vec::new() };
        for ast in &togaki.items {
            let item = match ast.clone() {
                AST::Action(a) => SerifItem::Action(a),
                AST::Require(a) => SerifItem::Require(a),
                AST::Either(a) => SerifItem::Either(a),
                AST::Forget(a) => SerifItem::Forget(a),
                AST::Memory(a) => SerifItem::Memory(a),
                AST::Serif(a) => SerifItem::Serif(a),
                AST::Comment(a) => SerifItem::Comment(a),
                AST::Error(a) => SerifItem::Error(a),
                _ => panic!("push_serif: ast={:?}", ast),
            };
            l4.items.push(item)
        }
        self.l4 = Some(l4);
        self
    }
}

/// script to root block
pub fn scan(script: &Script) -> RootBlock {
    let mut builder = Builder::new();
    for ast in script {
        builder = match ast {
            AST::DocComment(_) => builder.push_doc_comment(ast),
            AST::Comment(_) => builder.push_comment(ast),
            AST::Error(_) => builder.push_error(ast),
            AST::Hasira(h) => {
                if h.level == 0 {
                    builder.push_actor(&h)
                } else {
                    builder.push_hasira(ast)
                }
            }
            AST::Anchor(_) => builder.push_anchor(ast),
            AST::ShortJump(_) => builder.push_jump(ast),
            AST::LongJump(_) => builder.push_jump(ast),
            AST::Togaki(t) => builder.push_serif(t),
            _ => builder,
        }
    }
    builder.root()
}
