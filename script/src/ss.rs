use rhai::{ImmutableString, StaticVec};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;

use crate::ss_fmt as SS;

#[derive(Clone, Default, Debug)]
struct ActorState {
    pub id: ImmutableString,
    pub name: ImmutableString,
    pub talk_char_count: usize,
    pub is_first_talk: bool,
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
        actors: StaticVec<(S, S)>,
        default_emote: S,
    ) -> SakuraScriptBuilder {
        let actors = actors
            .into_iter()
            .map(|a| {
                let (name, id) = a;
                let name = name.into();
                let id = id.into();
                ActorState {
                    id: id,
                    name: name,
                    talk_char_count: 0,
                    is_first_talk: false,
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

    #[inline]
    fn now_actor_mut(&mut self) -> &mut ActorState {
        &mut self.actors[self.now_actor_index]
    }
    #[inline]
    fn now_actor(&self) -> &ActorState {
        &self.actors[self.now_actor_index]
    }

    /// actorのindexを名前から検索。
    fn get_actor_index_by_name<S: Borrow<ImmutableString>>(
        &self,
        name: S,
    ) -> Result<usize, String> {
        self.actors
            .iter()
            .enumerate()
            .filter(|a| a.1.name == *name.borrow())
            .map(|a| a.0)
            .next()
            .ok_or_else(|| format!("not found actor_name '{}'", name.borrow()))
    }

    /// actor の変更
    pub fn change_actor<S: Borrow<ImmutableString> + Clone>(
        &mut self,
        actor_name: S,
    ) -> Result<(), String> {
        let index = self.get_actor_index_by_name(actor_name)?;
        self.now_actor_index = index;
        let is_first_talk = !self.has_first_talk;
        self.has_first_talk = true;
        let mut actor = self.now_actor_mut();
        actor.is_first_talk = is_first_talk;
        actor.talk_char_count = 0;
        let actor = self.now_actor();

        self.buf = format!("{}{}", self.buf, &actor.id);
        Ok(())
    }

    pub fn emote<S: AsRef<str>>(&mut self, text: S) -> Result<(), String> {
        self.buf = format!("{}\\s[{}]", self.buf, text.as_ref());
        Ok(())
    }

    pub fn new_line(&mut self, percent: i32) -> Result<(), String> {
        self.buf = format!("{}\\n[{}]", self.buf, percent);
        Ok(())
    }
}
