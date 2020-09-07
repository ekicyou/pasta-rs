use crate::error::*;
use crate::ss::builder::SakuraScriptBuilder;
pub use crate::ss::env::*;
use crate::ss::yield_redume;
use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, ImmutableString, Map, RegisterFn, AST};
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

/// シーン情報
#[derive(Debug, Default, Clone)]
pub struct Scene {
    fn_ptr: FnPtr,
}

#[derive(Debug)]
pub struct ScenePlayer {
    scene: Scene,
    req: EnvDic,
    actor: EnvDic,
    app: EnvDic,
    save: EnvDic,
    eng: Rc<RefCell<Engine>>,
    ast: Rc<RefCell<AST>>,
    builder: SakuraScriptBuilder,
}

impl Iterator for ScenePlayer {
    type Item = ImmutableString;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl ScenePlayer {
    pub fn new(
        scene: Scene,
        builder: SakuraScriptBuilder,
        eng: Rc<RefCell<Engine>>,
        ast: Rc<RefCell<AST>>,
    ) -> Self {
        Self {
            scene: scene,
            req: Default::default(),
            actor: Default::default(),
            app: Default::default(),
            save: Default::default(),
            eng: eng,
            ast: ast,
            builder: builder,
        }
    }

    /// シーンをカットし、１シーン確定します。
    fn cut(&mut self) -> bool {
        let script = "スクリプト";
        true
    }
}
