use crate::error::*;
use crate::ss::fmt::*;
use rhai::{ImmutableString, StaticVec};
use std::fmt::{Display, Write};
use std::iter::IntoIterator;

#[derive(Clone, Debug)]
struct ActorState {
    pub id: ImmutableString,
    pub name: ImmutableString,
    pub talk_len: usize,
    pub is_first_talk: bool,
    pub is_talked: bool,
    pub has_new_line: bool,
    pub new_line: usize,
}

#[derive(Clone, Debug)]
pub struct SakuraScriptBuilder {
    default_emote: ImmutableString,
    actors: StaticVec<ActorState>,
    now_actor_index: usize,
    has_first_talk: bool,
    buf: String,
}

impl SakuraScriptBuilder {
    pub fn new<S: Into<ImmutableString> + Clone, U: Into<ImmutableString>>(
        actors: StaticVec<(S, S, usize)>,
        default_emote: U,
    ) -> SakuraScriptBuilder {
        let actors = actors
            .into_iter()
            .map(|a| {
                let (name, id, new_line) = a;
                let name = name.into();
                let id = id.into();
                ActorState {
                    id: id,
                    name: name,
                    talk_len: 0,
                    is_first_talk: false,
                    is_talked: false,
                    has_new_line: false,
                    new_line: new_line,
                }
            })
            .collect();
        SakuraScriptBuilder {
            default_emote: default_emote.into(),
            actors: actors,
            buf: Default::default(),
            has_first_talk: true,
            now_actor_index: 0,
        }
    }

    fn reset(&mut self) -> ImmutableString {
        let rc = {
            let mut prefix = String::new();
            for actor in self.actors.iter() {
                if !actor.is_first_talk && actor.is_talked {
                    prefix.write_fmt(format_args!("{}", actor.id));
                    prefix.write_surface(&self.default_emote);
                }
            }
            prefix.write_fmt(format_args!("{}\\e", &self.buf));
            prefix.into()
        };
        self.buf = Default::default();
        self.has_first_talk = true;
        self.now_actor_index = 0;
        for actor in self.actors.iter_mut() {
            actor.is_first_talk = false;
            actor.is_talked = false;
            actor.has_new_line = false;
            actor.talk_len = 0;
        }
        rc
    }

    /// actorのindexを名前から検索。
    fn get_actor_index_by_name<S: Into<ImmutableString>>(&self, name: S) -> PastaResult<usize> {
        let name = name.into();
        self.actors
            .iter()
            .enumerate()
            .filter(|a| a.1.name == name)
            .map(|a| a.0)
            .next()
            .ok_or_else(|| PastaError::ActorNotFound(name.clone()))
    }

    /// A: actor 切り替え
    pub fn change_actor<S: Into<ImmutableString>>(&mut self, name: S) -> PastaResult<()> {
        let name = name.into();
        let index = self.get_actor_index_by_name(name)?;
        self.now_actor_index = index;
        let mut actor = &mut self.actors[self.now_actor_index];
        actor.has_new_line = actor.talk_len > 0;
        actor.talk_len = 0;
        actor.has_new_line = true;
        self.buf.write_fmt(format_args!("{}", actor.id))?;
        Ok(())
    }

    /// E: 表情の変更
    pub fn emote<S: Display>(&mut self, value: S) -> PastaResult<()> {
        self.buf.write_surface(value)?;
        Ok(())
    }

    /// 強制改行(100%)
    pub fn br(&mut self) -> PastaResult<()> {
        self.buf.write_new_line(100)?;
        Ok(())
    }
    /// 以後の改行幅を変更
    pub fn change_new_line(&mut self, percent: usize) -> PastaResult<()> {
        let mut actor = &mut self.actors[self.now_actor_index];
        actor.new_line = percent;
        Ok(())
    }
    /// トーク
    pub fn talk<S: AsRef<str>>(&mut self, talk: S) -> PastaResult<()> {
        let new_line = {
            let mut actor = &mut self.actors[self.now_actor_index];
            let mut new_line = if actor.has_new_line {
                actor.new_line
            } else {
                0
            };
            let len = talk.as_ref().len();
            if len == 0 {
                new_line = 0;
            } else {
                actor.is_talked = true;
                actor.has_new_line = false;
                if self.has_first_talk {
                    actor.is_first_talk = true;
                    self.has_first_talk = false;
                }
            }
            actor.talk_len += len;
            new_line
        };
        self.buf.write_new_line(new_line)?;

        self.buf.write_talk(talk)?;
        Ok(())
    }

    /// B: 改行してトーク。
    /// ただし、セリフ冒頭の場合は改行しない。
    pub fn br_t<S: AsRef<str>>(&mut self, talk: S) -> PastaResult<()> {
        let has_br = {
            let actor = &self.actors[self.now_actor_index];
            let len = talk.as_ref().len();
            len > 0 && actor.talk_len != 0
        };
        if has_br {
            self.br()?;
        }
        self.talk(talk)
    }

    /// スクリプト出力
    pub fn build(&mut self) -> PastaResult<ImmutableString> {
        Ok(self.reset())
    }
}
