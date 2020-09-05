use crate::error::*;
use crate::ss::builder::SakuraScriptBuilder;
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Map, RegisterFn, AST};
use std::cell::RefCell;
use std::fmt::Display;
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

    /// A: actor 切り替え
    pub fn change_actor<S: Into<ImmutableString>>(&mut self, name: S) -> PastaResult<()> {
        self.builder.change_actor(name)
    }

    /// E: 表情の変更
    pub fn emote<S: Display>(&mut self, value: S) -> PastaResult<()> {
        self.builder.emote(value)
    }

    /// 強制改行(100%)
    pub fn br(&mut self) -> PastaResult<()> {
        self.builder.br()
    }
    /// 以後の改行幅を変更
    pub fn change_new_line(&mut self, percent: usize) -> PastaResult<()> {
        self.builder.change_new_line(percent)
    }
    /// トーク
    pub fn talk<S: AsRef<str>>(&mut self, talk: S) -> PastaResult<()> {
        self.builder.talk(talk)
    }

    /// B: 改行してトーク。
    /// ただし、セリフ冒頭の場合は改行しない。
    pub fn br_t<S: AsRef<str>>(&mut self, talk: S) -> PastaResult<()> {
        self.builder.br_t(talk)
    }

    /// A: actor 切り替え
    fn rhai_change_actor(mut self, name: ImmutableString) -> Self {
        if let Err(e) = self.change_actor(name) {
            log::error!("{}", e)
        }
        self
    }

    /// E: 表情の変更
    fn rhai_emote(mut self, value: ImmutableString) -> Self {
        if let Err(e) = self.emote(value) {
            log::error!("{}", e)
        }
        self
    }

    /// 以後の改行幅を変更
    fn rhai_change_new_line(mut self, percent: usize) -> Self {
        if let Err(e) = self.change_new_line(percent) {
            log::error!("{}", e)
        }
        self
    }
    /// トーク
    fn rhai_talk(mut self, talk: ImmutableString) -> Self {
        if let Err(e) = self.talk(talk.as_str()) {
            log::error!("{}", e)
        }
        self
    }

    /// B: 改行してトーク。
    /// ただし、セリフ冒頭の場合は改行しない。
    fn rhai_br_t(mut self, talk: ImmutableString) -> Self {
        if let Err(e) = self.br_t(talk.as_str()) {
            log::error!("{}", e)
        }
        self
    }

    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_get("actor", Self::rhai_actor);
        eng.register_get("req", Self::rhai_req);
        eng.register_get("app", Self::rhai_app);
        eng.register_get("save", Self::rhai_save);

        eng.register_fn("A", Self::rhai_change_actor);
        eng.register_fn("E", Self::rhai_emote);
        eng.register_fn("L", Self::rhai_change_new_line);
        eng.register_fn("T", Self::rhai_talk);
        eng.register_fn("B", Self::rhai_br_t);
        eng.register_custom_operator("A", 2)?;
        eng.register_custom_operator("E", 2)?;
        eng.register_custom_operator("L", 2)?;
        eng.register_custom_operator("T", 2)?;
        eng.register_custom_operator("B", 2)?;
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
