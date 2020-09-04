use crate::error::*;
use crate::ss::builder::SakuraScriptBuilder;
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Map, RegisterFn, AST};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct EnvDic {
    pub(crate) dic: Rc<RefCell<Map>>,
}
impl Default for EnvDic {
    fn default() -> Self {
        EnvDic {
            dic: Rc::new(RefCell::new(Map::new())),
        }
    }
}
impl EnvDic {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<K: Into<ImmutableString>>(&self, key: K) -> Dynamic {
        let key = key.into();
        let dic = (*self.dic).borrow();
        match dic.get(&key) {
            Some(v) => v.clone(),
            _ => ().into(),
        }
    }
    pub fn insert<K: Into<ImmutableString>, D: Into<Dynamic>>(
        &mut self,
        key: K,
        value: D,
    ) -> Dynamic {
        let key = key.into();
        let mut dic = (*self.dic).borrow_mut();
        match dic.insert(key, value.into()) {
            Some(v) => v,
            _ => ().into(),
        }
    }

    pub fn remove<K: Into<ImmutableString>>(&mut self, key: K) -> Dynamic {
        let key = key.into();
        let mut dic = (*self.dic).borrow_mut();
        match dic.remove(&key) {
            Some(v) => v,
            _ => ().into(),
        }
    }

    fn rhai_get(&mut self, key: ImmutableString) -> Dynamic {
        self.get(key)
    }
    fn rhai_set(&mut self, key: ImmutableString, value: Dynamic) {
        self.insert(key, value);
    }
    fn rhai_remove(&mut self, key: ImmutableString) -> Dynamic {
        self.remove(key)
    }

    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_indexer_get_set(Self::rhai_get, Self::rhai_set);
        eng.register_fn("remove", Self::rhai_remove);
        Ok(())
    }
}

/// 再生環境
#[derive(Debug, Clone)]
pub struct PlayEnv {
    pub(crate) req: EnvDic,
    pub(crate) actor: EnvDic,
    pub(crate) app: EnvDic,
    pub(crate) save: EnvDic,
    pub(crate) eng: Rc<RefCell<Engine>>,
    pub(crate) ast: Rc<RefCell<AST>>,
    pub(crate) builder: SakuraScriptBuilder,
}

impl PlayEnv {
    pub fn new(
        eng: Rc<RefCell<Engine>>,
        ast: Rc<RefCell<AST>>,
        builder: SakuraScriptBuilder,
    ) -> Self {
        Self {
            req: Default::default(),
            actor: Default::default(),
            app: Default::default(),
            save: Default::default(),
            eng: eng,
            ast: ast,
            builder: builder,
        }
    }

    pub fn actor(&self) -> EnvDic {
        self.actor.clone()
    }
    pub fn req(&self) -> EnvDic {
        self.req.clone()
    }
    pub fn app(&self) -> EnvDic {
        self.app.clone()
    }
    pub fn save(&self) -> EnvDic {
        self.save.clone()
    }

    pub fn rhai_actor(&mut self) -> EnvDic {
        self.actor()
    }
    pub fn rhai_req(&mut self) -> EnvDic {
        self.req()
    }
    pub fn rhai_app(&mut self) -> EnvDic {
        self.app()
    }
    pub fn rhai_save(&mut self) -> EnvDic {
        self.save()
    }
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_get("actor", Self::rhai_actor);
        eng.register_get("req", Self::rhai_req);
        eng.register_get("app", Self::rhai_app);
        eng.register_get("save", Self::rhai_save);
        Ok(())
    }
}
pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
    EnvDic::register_rhai(eng)?;
    PlayEnv::register_rhai(eng)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dic_test() -> PastaResult<()> {
        let mut dic = EnvDic::new();
        {
            dic.insert("hello", "world");
            let v = dic.get("hello");
            assert_eq!(v.as_str()?, "world");
        }
        {
            let v = dic.get("no");
            let _ = v.cast::<()>();
        }
        {
            let mut dic2 = dic.clone();
            dic2.insert("明日の", "天気");
            let v = dic.get("明日の");
            assert_eq!(v.as_str()?, "天気");
        }
        {
            let mut dic2 = dic.clone();
            dic2.remove("明日の");
            let v = dic.get("明日の");
            let _ = v.cast::<()>();
        }
        Ok(())
    }
}
