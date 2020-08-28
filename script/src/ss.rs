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
    pub talk_char_len: usize,
    pub is_first_talk: bool,
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
    pub fn new<S: Into<ImmutableString>>(
        actors: StaticVec<(S, S, usize)>,
        default_emote: S,
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
                    talk_char_len: 0,
                    is_first_talk: false,
                    new_line: new_line,
                }
            })
            .collect();
        SakuraScriptBuilder {
            default_emote: default_emote.into(),
            actors: actors,
            buf: Default::default(),
            has_first_talk: false,
            now_actor_index: 0,
        }
    }

    fn reset(&mut self) -> ImmutableString {
        let rc = self.buf.clone().into();
        self.buf = Default::default();
        rc
    }

    #[inline]
    fn now_actor_mut(&mut self) -> &mut ActorState {
        &mut self.actors[self.now_actor_index]
    }
    #[inline]
    fn now_actor(&self) -> &ActorState {
        &self.actors[self.now_actor_index]
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
        {
            let mut actor = self.now_actor_mut();
            actor.is_first_talk = is_first_talk;
            actor.talk_char_len = 0;
        }
        self.buf
            .write_fmt(format_args!("{}", self.actors[self.now_actor_index].id))?;
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
        self.buf.write_talk(talk)?;
        Ok(())
    }

    /// スクリプト合成
    pub fn build(&mut self) -> PastaResult<ImmutableString> {
        Ok(self.reset())
    }
}
