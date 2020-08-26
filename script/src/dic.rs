//! # rhaiを利用した辞書構造
//!
//! パスタスクリプトは、rhaiスクリプトとしてDSL変換される。
//!
//! ```pasta
//! ＠＠　！通常会話
//!
//! ＠ダックが帰ってくる！
//!
//! むらさき  ＠興奮笑顔
//!         アヒルやアヒル！
//!         11年ぶり、大阪にアヒルが
//!         帰ってくるねんで！
//!
//! えも    ＠冷笑
//!         ‥‥１１年ぶりって、
//!         君生まれてたの？
//! ```
//!
//! ```rhai
//! let h = hasira();
//!
//!{   // @@@@
//!    let h = h;
//!    {   // @@@
//!        let h = h;
//!        {   // @@
//!            let h = h;
//!            h.condition = cond(has("通常会話"));
//!            {   // @
//!                let h = h;
//!                h.title = "ダックが帰ってくる！";
//!                h.script = |a| {
//!                    a.むらさき                      // キャラクタを指定
//!                        E "興奮笑顔"                // E, Tはカスタムオペレータ
//!                        T "アヒルやアヒル！"
//!                        T "11年ぶり、大阪にアヒルが"
//!                        T "帰ってくるねんで！"
//!                        ;
//!                    a.えも
//!                        E "冷笑"
//!                        T "11年ぶりって、"
//!                        T "君、生まれてたの？"
//!                        ;
//!                        a.action;                   // アクション（1回のトーク）rustに制御が返る。
//!                    }
//!                };
//!            }
//!        }
//!    }
//!}
//! ```
use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, ImmutableString, Map, Module, RegisterFn};
use std::any::Any;
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;

pub type FuncReturn<T> = Result<T, Box<EvalAltResult>>;

/// 条件式
#[derive(Clone, Debug)]
pub enum ConditionExpr {
    /// キーワードが存在すること
    Has(ImmutableString),

    /// キーワードの値がi32範囲内
    RangeI32(ImmutableString, Option<i32>, Option<i32>),

    /// キーワードの値がi64範囲内
    RangeI64(ImmutableString, Option<i64>, Option<i64>),

    /// キーワードの値がf32範囲内
    RangeF32(ImmutableString, Option<f32>, Option<f32>),

    /// キーワードの値がf64範囲内
    RangeF64(ImmutableString, Option<f64>, Option<f64>),

    /// キーワードの値が文字列範囲内
    RangeString(
        ImmutableString,
        Option<ImmutableString>,
        Option<ImmutableString>,
    ),

    /// いずれかの Self が成立すること
    Or(Box<Self>, Box<Self>),

    /// 両方の Self が成立すること
    And(Box<Self>, Box<Self>),
}

impl ConditionExpr {
    /// コンストラクタ
    pub fn has<S: Into<ImmutableString>>(key: S) -> Self {
        Self::Has(key.into())
    }
    pub fn range_i32<S: Into<ImmutableString>>(key: S, min: Option<i32>, max: Option<i32>) -> Self {
        Self::RangeI32(key.into(), min, max)
    }
    pub fn range_i64<S: Into<ImmutableString>>(key: S, min: Option<i64>, max: Option<i64>) -> Self {
        Self::RangeI64(key.into(), min, max)
    }
    pub fn range_f32<S: Into<ImmutableString>>(key: S, min: Option<f32>, max: Option<f32>) -> Self {
        Self::RangeF32(key.into(), min, max)
    }
    pub fn range_f64<S: Into<ImmutableString>>(key: S, min: Option<f64>, max: Option<f64>) -> Self {
        Self::RangeF64(key.into(), min, max)
    }
    pub fn range_str<S: Into<ImmutableString>>(
        key: ImmutableString,
        min: Option<S>,
        max: Option<S>,
    ) -> Self {
        Self::RangeString(key, min.map(|a| a.into()), max.map(|a| a.into()))
    }
    pub fn or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }
    pub fn and(left: Self, right: Self) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_fn("has", Self::has::<ImmutableString>);
        eng.register_fn("range_i32", Self::range_i32::<ImmutableString>);
        eng.register_fn("range_i64", Self::range_i64::<ImmutableString>);
        eng.register_fn("range_f32", Self::range_f32::<ImmutableString>);
        eng.register_fn("range_f64", Self::range_f64::<ImmutableString>);
        eng.register_fn("range_str", Self::range_str::<ImmutableString>);
        eng.register_fn("or", Self::or);
        eng.register_fn("and", Self::and);
        Ok(())
    }
}

/// 条件と解放時処理
#[derive(Clone, Debug)]
pub struct Condition {
    expr: ConditionExpr,
    finally: Option<FnPtr>,
}
impl Condition {
    /// コンストラクタ
    pub fn new(expr: ConditionExpr, finally: Option<FnPtr>) -> Self {
        Self {
            expr: expr,
            finally: finally,
        }
    }
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_fn("cond", |expr| Self::new(expr, None));
        eng.register_fn("cond", |expr, finally| Self::new(expr, Some(finally)));
        Ok(())
    }
}

/// ビルダーコールバック
#[derive(Clone, Default, Debug)]
pub struct PlayBuilderCallbackItem {
    fn_actor: FnPtr,
    fn_emote: FnPtr,
    fn_talk: FnPtr,
    fn_word: FnPtr,
}
impl PlayBuilderCallbackItem {
    fn set_fn_actor(&mut self, p: FnPtr) {
        self.fn_actor = p;
    }
    fn set_fn_emote(&mut self, p: FnPtr) {
        self.fn_emote = p;
    }
    fn set_fn_talk(&mut self, p: FnPtr) {
        self.fn_talk = p;
    }
    fn set_fn_word(&mut self, p: FnPtr) {
        self.fn_word = p;
    }

    fn fn_actor(&self) -> FnPtr {
        self.fn_actor.clone()
    }
    fn fn_emote(&self) -> FnPtr {
        self.fn_emote.clone()
    }
    fn fn_talk(&self) -> FnPtr {
        self.fn_talk.clone()
    }
    fn fn_word(&self) -> FnPtr {
        self.fn_word.clone()
    }

    /// emote 適用
    fn actor<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &Module,
        text: S,
    ) -> FuncReturn<()> {
        let a1 = Dynamic::from(text.into());
        let _ = self.fn_actor.call_dynamic(engine, lib, None, [a1])?;
        Ok(())
    }

    /// emote 適用
    fn emote<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &Module,
        text: S,
    ) -> FuncReturn<()> {
        let a1 = Dynamic::from(text.into());
        let _ = self.fn_emote.call_dynamic(engine, lib, None, [a1])?;
        Ok(())
    }

    /// talk 適用
    fn talk<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &Module,
        text: S,
    ) -> FuncReturn<()> {
        let a1 = Dynamic::from(text.into());
        let _ = self.fn_talk.call_dynamic(engine, lib, None, [a1])?;
        Ok(())
    }

    /// word 取得後、talk 適用
    fn word<S: Into<ImmutableString>>(
        &self,
        engine: &Engine,
        lib: &Module,
        text: S,
    ) -> FuncReturn<()> {
        let a1 = Dynamic::from(text.into());
        let word = {
            let dy = self.fn_word.call_dynamic(engine, lib, None, [a1])?;
            dy.take_string()?
        };
        self.talk(engine, lib, word)
    }
}

pub trait PlayBuilderCallbackAccess: Any + Clone {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem;
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem;

    fn set_fn_actor(&mut self, p: FnPtr) {
        self.playbuilder_callback_item_mut().set_fn_actor(p)
    }
    fn set_fn_emote(&mut self, p: FnPtr) {
        self.playbuilder_callback_item_mut().set_fn_emote(p)
    }
    fn set_fn_talk(&mut self, p: FnPtr) {
        self.playbuilder_callback_item_mut().set_fn_talk(p)
    }
    fn set_fn_word(&mut self, p: FnPtr) {
        self.playbuilder_callback_item_mut().set_fn_word(p)
    }

    fn fn_actor(&self) -> FnPtr {
        self.playbuilder_callback_item().fn_actor()
    }
    fn fn_emote(&self) -> FnPtr {
        self.playbuilder_callback_item().fn_emote()
    }
    fn fn_talk(&self) -> FnPtr {
        self.playbuilder_callback_item().fn_talk()
    }
    fn fn_word(&self) -> FnPtr {
        self.playbuilder_callback_item().fn_word()
    }

    fn rhai_fn_actor(&mut self) -> FnPtr {
        self.fn_actor()
    }
    fn rhai_fn_emote(&mut self) -> FnPtr {
        self.fn_emote()
    }
    fn rhai_fn_talk(&mut self) -> FnPtr {
        self.fn_talk()
    }
    fn rhai_fn_word(&mut self) -> FnPtr {
        self.fn_word()
    }
    /// rhaiへの登録
    fn register_rhai_callback_access(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_get_set("fn_actor", Self::rhai_fn_actor, Self::set_fn_actor);
        eng.register_get_set("fn_emote", Self::rhai_fn_emote, Self::set_fn_emote);
        eng.register_get_set("fn_talk", Self::rhai_fn_talk, Self::set_fn_emote);
        eng.register_get_set("fn_word", Self::rhai_fn_word, Self::set_fn_emote);
        Ok(())
    }
}

pub trait PlayBuilderCallback: PlayBuilderCallbackAccess {
    fn get_rhai_env(&self) -> (&Engine, &Module);

    /// actor 適用
    fn actor<S: Into<ImmutableString>>(self, text: S) -> Self {
        let cb = self.playbuilder_callback_item();
        let (engine, lib) = self.get_rhai_env();
        if let Err(e) = cb.actor(engine, lib, text) {
            log::error!("{:?}", e)
        }
        self
    }
    /// emote 適用
    fn emote<S: Into<ImmutableString>>(self, text: S) -> Self {
        let cb = self.playbuilder_callback_item();
        let (engine, lib) = self.get_rhai_env();
        if let Err(e) = cb.emote(engine, lib, text) {
            log::error!("{:?}", e)
        }
        self
    }

    /// talk 適用
    fn talk<S: Into<ImmutableString>>(self, text: S) -> Self {
        let cb = self.playbuilder_callback_item();
        let (engine, lib) = self.get_rhai_env();
        if let Err(e) = cb.talk(engine, lib, text) {
            log::error!("{:?}", e)
        }
        self
    }

    /// word 取得後、talk 適用
    fn word<S: Into<ImmutableString>>(self, text: S) -> Self {
        let cb = self.playbuilder_callback_item();
        let (engine, lib) = self.get_rhai_env();
        if let Err(e) = cb.word(engine, lib, text) {
            log::error!("{:?}", e)
        }
        self
    }
    /// rhaiへの登録
    fn register_rhai_callback(eng: &mut Engine) -> Result<(), String> {
        Self::register_rhai_callback_access(eng)?;
        eng.register_fn("A", Self::actor::<ImmutableString>);
        eng.register_fn("E", Self::emote::<ImmutableString>);
        eng.register_fn("T", Self::talk::<ImmutableString>);
        eng.register_fn("W", Self::word::<ImmutableString>);
        Ok(())
    }
}

/// 脚本
#[derive(Clone, Default, Debug)]
pub struct ScreenPlay {
    scene: Vec<Scene>,
    cb: PlayBuilderCallbackItem,
    actors: HashMap<ImmutableString, Actor>,
}

impl ScreenPlay {
    /// コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.scene.len()
    }

    fn count_rhai(&mut self) -> i32 {
        self.scene.len() as i32
    }
    /// シーンを一つ追加する。
    pub fn push(&mut self, hasira: Hasira, play: FnPtr) -> Result<(), String> {
        let scene = Scene::new(hasira, play);
        self.scene.push(scene);
        Ok(())
    }

    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_fn("screen_play", Self::new);
        eng.register_get("count", Self::count_rhai);
        eng.register_fn("push", Self::push);
        Ok(())
    }
}

impl PlayBuilderCallbackAccess for ScreenPlay {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem {
        &self.cb
    }
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem {
        &mut self.cb
    }
}

/// 柱
#[derive(Clone, Default, Debug)]
pub struct Hasira {
    title: ImmutableString,
    condition: Vec<Condition>,
    cb: PlayBuilderCallbackItem,
}

impl fmt::Display for Hasira {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"( title: {},
condition: {:?}
)"#,
            self.title, self.condition
        )
    }
}

impl Hasira {
    /// コンストラクタ
    pub fn new() -> Self {
        Default::default()
    }
}

impl Hasira {
    pub fn title(&mut self) -> ImmutableString {
        self.title.clone()
    }
    pub fn set_title(&mut self, value: ImmutableString) -> () {
        self.title = value;
    }
    pub fn push_condition(&mut self, value: Condition) -> () {
        self.condition.push(value);
    }
}

impl Hasira {
    pub fn print(&mut self) -> ImmutableString {
        format!("{}", self).into()
    }
    pub fn debug(&mut self) -> ImmutableString {
        format!("{:?}", self).into()
    }
    pub fn join_str_obj(s1: ImmutableString, s2: Self) -> ImmutableString {
        format!("{}{}", s1, s2).into()
    }
    pub fn join_obj_obj(s1: Self, s2: ImmutableString) -> ImmutableString {
        format!("{}{}", s1, s2).into()
    }
}
impl Hasira {
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_fn("print", Self::print);
        eng.register_fn("debug", Self::debug);
        eng.register_fn("hasira", Self::new);
        eng.register_fn("push_condition", Self::push_condition);
        eng.register_get_set("title", Self::title, Self::set_title);
        eng.register_set("condition", Self::push_condition);
        Ok(())
    }
}

impl PlayBuilderCallbackAccess for Hasira {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem {
        &self.cb
    }
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem {
        &mut self.cb
    }
}

/// シーン
#[derive(Clone, Default, Debug)]
pub struct Scene {
    hasira: Hasira,
    play: FnPtr,
}
impl Scene {
    /// コンストラクタ
    pub fn new(hasira: Hasira, play: FnPtr) -> Self {
        Self {
            hasira: hasira,
            play: play,
        }
    }
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        Ok(())
    }
}

impl PlayBuilderCallbackAccess for Scene {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem {
        self.hasira.playbuilder_callback_item()
    }
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem {
        self.hasira.playbuilder_callback_item_mut()
    }
}

/// 脚本ビルダ
#[derive(Clone, Debug)]
pub struct PlayBuilder {
    engine: Rc<Engine>,
    lib: Rc<Module>,
    actors: HashMap<ImmutableString, Rc<Actor>>,
    cb: PlayBuilderCallbackItem,
    now_actor: Option<Rc<Actor>>,
    tag: Option<Map>,
}

impl PlayBuilderCallbackAccess for PlayBuilder {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem {
        &self.cb
    }
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem {
        &mut self.cb
    }
}
impl PlayBuilderCallback for PlayBuilder {
    fn get_rhai_env(&self) -> (&Engine, &Module) {
        (&self.engine, &self.lib)
    }
}

impl PlayBuilder {
    /// コンストラクタ
    pub fn new(
        engine: Rc<Engine>,
        lib: Rc<Module>,
        screen_play: &ScreenPlay,
        scene: &Scene,
        tag: Option<Map>,
    ) -> Self {
        let actors: HashMap<_, _> = (screen_play.actors)
            .values()
            .map(|a| (ImmutableString::from(a.name()), Rc::new(a.clone())))
            .collect();

        let now_actor = actors.values().next().cloned();
        let cb = scene.playbuilder_callback_item().clone();
        PlayBuilder {
            engine: engine,
            lib: lib,
            actors: actors,
            cb: cb,
            tag: tag,
            now_actor: now_actor,
        }
    }

    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        Self::register_rhai_callback(eng)?;
        eng.register_custom_operator("A", 2)?;
        eng.register_custom_operator("E", 2)?;
        eng.register_custom_operator("T", 2)?;
        eng.register_custom_operator("W", 2)?;
        Ok(())
    }
}

/// 役者
#[derive(Clone, Default, Debug)]
pub struct Actor {
    name: ImmutableString,
    tag: Option<Map>,
    cb: PlayBuilderCallbackItem,
}

impl Actor {
    /// コンストラクタ
    pub fn new<S: Into<ImmutableString>>(name: S, tag: Option<Map>) -> Self {
        Self {
            name: name.into(),
            tag: tag,
            cb: Default::default(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }3

    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
        eng.register_type::<Self>();
        eng.register_fn("actor", |name: ImmutableString| Self::new(name, None));
        eng.register_fn("actor", |name: ImmutableString, tag: Map| {
            Self::new(name, Some(tag))
        });
        Ok(())
    }
}

impl PlayBuilderCallbackAccess for Actor {
    fn playbuilder_callback_item(&self) -> &PlayBuilderCallbackItem {
        &self.cb
    }
    fn playbuilder_callback_item_mut(&mut self) -> &mut PlayBuilderCallbackItem {
        &mut self.cb
    }
}

pub fn register_rhai(eng: &mut Engine) -> Result<(), String> {
    ConditionExpr::register_rhai(eng)?;
    Condition::register_rhai(eng)?;
    Hasira::register_rhai(eng)?;
    ScreenPlay::register_rhai(eng)?;
    Scene::register_rhai(eng)?;
    Actor::register_rhai(eng)?;
    PlayBuilder::register_rhai(eng)?;

    Ok(())
}
