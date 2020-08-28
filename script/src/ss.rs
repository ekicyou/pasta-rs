use crate::error::*;
use crate::ss_fmt::*;
use rhai::{ImmutableString, StaticVec};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::fmt::{Display, Write};

#[derive(Clone, Default, Debug)]
struct ActorState {
    pub id: ImmutableString,
    pub name: ImmutableString,
    pub talk_len: usize,
    pub is_first_talk: bool,
    pub has_new_line: bool,
    pub new_line: usize,
}

#[derive(Clone, Default, Debug)]
pub struct SakuraScriptBuilder {
    default_emote: ImmutableString,
    actors: StaticVec<ActorState>,
    now_actor_index: usize,
    has_first_talk: bool,
    buf: String,
}

impl SakuraScriptBuilder {
    pub fn new<S: Into<ImmutableString> + Clone>(
        actors: &[(S, S, usize)],
        default_emote: S,
    ) -> SakuraScriptBuilder {
        let actors = actors
            .into_iter()
            .map(|a| {
                let (name, id, new_line) = a;
                let name = (*name).clone().into();
                let id = (*id).clone().into();
                ActorState {
                    id: id,
                    name: name,
                    talk_len: 0,
                    is_first_talk: false,
                    has_new_line: false,
                    new_line: *new_line,
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
                if !actor.is_first_talk {
                    prefix.write_fmt(format_args!("{}", actor.id));
                    prefix.write_surface(&self.default_emote);
                }
            }
            prefix.write_str(&self.buf);
            prefix.into()
        };
        self.buf = Default::default();
        self.has_first_talk = true;
        self.now_actor_index = 0;
        for actor in self.actors.iter_mut() {
            actor.is_first_talk = false;
            actor.has_new_line = false;
            actor.talk_len = 0;
        }
        rc
    }

    /// actorのindexを名前から検索。
    fn get_actor_index_by_name<S: Borrow<ImmutableString>>(&self, name: S) -> PastaResult<usize> {
        self.actors
            .iter()
            .enumerate()
            .filter(|a| a.1.name == *name.borrow())
            .map(|a| a.0)
            .next()
            .ok_or_else(|| PastaError::ActorNotFound(name.borrow().clone()))
    }

    /// actor の変更
    pub fn change_actor<S: Borrow<ImmutableString> + Clone>(
        &mut self,
        actor_name: S,
    ) -> PastaResult<()> {
        let index = self.get_actor_index_by_name(actor_name)?;
        self.now_actor_index = index;
        let is_first_talk = !self.has_first_talk;
        self.has_first_talk = true;
        let mut actor = &mut self.actors[self.now_actor_index];
        actor.is_first_talk = is_first_talk;
        actor.has_new_line = actor.talk_len > 0;
        actor.talk_len = 0;
        actor.has_new_line = true;
        self.buf.write_fmt(format_args!("{}", actor.id))?;
        Ok(())
    }

    /// 表情の変更
    pub fn emote<S: Display>(&mut self, text: S) -> PastaResult<()> {
        self.buf.write_surface(text)?;
        Ok(())
    }

    /// 改行コード
    pub fn new_line(&mut self, percent: usize) -> PastaResult<()> {
        self.buf.write_new_line(percent)?;
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
                actor.has_new_line = false;
                if self.has_first_talk {
                    actor.is_first_talk = true;
                    self.has_first_talk = false;
                }
            }
            actor.talk_len += len;
            new_line
        };
        self.new_line(new_line)?;

        self.buf.write_talk(talk)?;
        Ok(())
    }

    /// スクリプト出力
    pub fn build(&mut self) -> PastaResult<ImmutableString> {
        Ok(self.reset())
    }
}
