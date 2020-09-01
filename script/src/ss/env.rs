use crate::error::*;
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Map, RegisterFn};
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

    pub fn reg_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_indexer_get_set(Self::rhai_get, Self::rhai_set);
        eng.register_fn("remove", Self::rhai_remove);
        Ok(())
    }
}

/// 再生環境
#[derive(Debug, Clone, Default)]
pub struct PlayEnv {
    pub(crate) actor: EnvDic,
    pub(crate) req: EnvDic,
    pub(crate) app: EnvDic,
    pub(crate) save: EnvDic,
}

impl PlayEnv {
    pub fn new() -> Self {
        Self::default()
    }
    fn actor(&self) -> EnvDic {
        self.actor.clone()
    }
    fn req(&self) -> EnvDic {
        self.req.clone()
    }
    fn app(&self) -> EnvDic {
        self.app.clone()
    }
    fn save(&self) -> EnvDic {
        self.save.clone()
    }
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
