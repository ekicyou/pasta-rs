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

use crate::error::*;
use rhai::{Engine, EvalAltResult, FnPtr, ImmutableString, Map, Module, RegisterFn, StaticVec};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;

pub type FuncReturn<T> = Result<T, Box<EvalAltResult>>;
pub type RhaiResult<T> = Result<T, String>;

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
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
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
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_fn("cond", |expr| Self::new(expr, None));
        eng.register_fn("cond", |expr, finally| Self::new(expr, Some(finally)));
        Ok(())
    }
}

/// 脚本
#[derive(Clone, Default, Debug)]
pub struct ScreenPlay {
    scene: Vec<Scene>,
    actors: HashMap<ImmutableString, Actor>,
}

impl ScreenPlay {
    /// コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }

    /// シーン登録数
    pub fn count(&self) -> usize {
        self.scene.len()
    }
    fn rhai_count(&mut self) -> i32 {
        self.scene.len() as i32
    }

    /// actor を追加します。
    pub fn push_actor(&mut self, actor: Actor) -> PastaResult<()> {
        let key = actor.name().into();
        self.actors.insert(key, actor);
        Ok(())
    }
    fn rhai_push_actor(&mut self, actor: Actor) -> RhaiResult<()> {
        self.push_actor(actor)?;
        Ok(())
    }

    /// シーンを一つ追加する。
    pub fn push(&mut self, hasira: Hasira, play: FnPtr) -> PastaResult<()> {
        let scene = Scene::new(hasira, play);
        self.scene.push(scene);
        Ok(())
    }
    fn rhai_push(&mut self, hasira: Hasira, play: FnPtr) -> RhaiResult<()> {
        self.push(hasira, play)?;
        Ok(())
    }
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_fn("screen_play", Self::new);
        eng.register_get("count", Self::rhai_count);
        eng.register_fn("push_actor", Self::rhai_push_actor);
        eng.register_fn("push", Self::rhai_push);
        Ok(())
    }
}

/// 柱
#[derive(Clone, Default, Debug)]
pub struct Hasira {
    title: ImmutableString,
    condition: Vec<Condition>,
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
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
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
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        Ok(())
    }
}

/// 脚本ビルダ
#[derive(Clone, Debug)]
pub struct PlayBuilder {
    engine: Rc<Engine>,
    lib: Rc<Module>,
    actors: HashMap<ImmutableString, Rc<Actor>>,
    now_actor_name: Option<ImmutableString>,
    script_builder: crate::ss::SakuraScriptBuilder,
    tag: Option<Map>,
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
        let script_builder = {
            let actors: StaticVec<_> = (screen_play.actors)
                .values()
                .map(|a| {
                    let name = ImmutableString::from(a.name());
                    let id = ImmutableString::from(a.id());
                    let new_line = a.new_line();
                    (name, id, new_line)
                })
                .collect();
            crate::ss::SakuraScriptBuilder::new(actors, "通常")
        };
        let actors: HashMap<_, _> = (screen_play.actors)
            .values()
            .map(|a| (ImmutableString::from(a.name()), Rc::new(a.clone())))
            .collect();
        let now_actor_name = actors.values().next().map(|a| a.name().into()).clone();
        let builder = PlayBuilder {
            engine: engine,
            lib: lib,
            actors: actors,
            tag: tag,
            now_actor_name: now_actor_name,
            script_builder: script_builder,
        };
        builder
    }

    /// A: actor 切り替え
    pub fn change_actor<S: Into<ImmutableString>>(&mut self, name: S) -> PastaResult<()> {
        let name = name.into();
        match self.actors.get(&name) {
            Some(actor) => {
                self.now_actor_name = Some(actor.name().into());
            }
            _ => Err(PastaError::ActorNotFound(name.clone()))?,
        }
        self.script_builder.change_actor(name)?;
        Ok(())
    }

    /// A: actor 切り替え
    fn rhai_change_actor<S: Into<ImmutableString>>(mut self, name: S) -> Self {
        if let Err(e) = self.change_actor(name) {
            log::error!("{:?}", e)
        }
        self
    }

    /// emote 切り替え
    pub fn emote<S: Borrow<ImmutableString>>(&mut self, text: S) -> PastaResult<()> {
        let text = text.borrow();
        match self.actors.get(text) {
            Some(actor) => {
                self.now_actor_name = Some(actor.name().into());
            }
            _ => Err(PastaError::ActorNotFound(text.clone()))?,
        }
        Ok(())
    }

    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_fn("A", Self::rhai_change_actor::<ImmutableString>);
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
    id: ImmutableString,
    name: ImmutableString,
    new_line: usize,
    tag: Map,
}

impl Actor {
    /// コンストラクタ
    pub fn new<S: Into<ImmutableString>>(
        id: S,
        name: S,
        new_line: usize,
        tag: Option<Map>,
    ) -> Self {
        let tag = match tag {
            Some(t) => t,
            None => Default::default(),
        };
        Self {
            id: id.into(),
            name: name.into(),
            new_line: new_line,
            tag: tag,
        }
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }
    pub fn id(&self) -> &str {
        self.id.borrow()
    }
    pub fn new_line(&self) -> usize {
        self.new_line
    }
    /// rhaiへの登録
    pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
        eng.register_type::<Self>();
        eng.register_fn(
            "actor",
            |id: ImmutableString, name: ImmutableString, new_line: isize| {
                Self::new(id, name, new_line as _, None)
            },
        );
        eng.register_fn(
            "actor",
            |id: ImmutableString, name: ImmutableString, new_line: isize, tag: Map| {
                Self::new(id, name, new_line as _, Some(tag))
            },
        );
        Ok(())
    }
}

pub fn register_rhai(eng: &mut Engine) -> PastaResult<()> {
    ConditionExpr::register_rhai(eng)?;
    Condition::register_rhai(eng)?;
    Hasira::register_rhai(eng)?;
    ScreenPlay::register_rhai(eng)?;
    Scene::register_rhai(eng)?;
    Actor::register_rhai(eng)?;
    PlayBuilder::register_rhai(eng)?;

    Ok(())
}
