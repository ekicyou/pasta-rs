use super::shiori::*;
use pest;
use std::collections::HashMap;

pub use super::shiori::Rule;

pub type Error<'a> = pest::Error<'a, Rule::req>;

/// SHIORI3リクエストの解析結果を格納します。
#[derive(PartialEq, Eq, Debug)]
pub struct ShioriRequest<'a> {
    pub version: i32,
    pub method: Rule,
    pub id: &'a str,
    pub sender: &'a str,
    pub security_level: &'a str,
    pub charset: &'a str,
    pub status: &'a str,
    pub base_id: &'a str,
    pub reference: Vec<&'a str>,
    pub dic: HashMap<String, &'a str>,
}

impl<'a> ShioriRequest<'a> {
    pub fn from_str(text: &'a str) -> Result<ShioriRequest<'a>, Error<'a>> {
        Err(Error<'a>::cu ::UnknownHeader)
    }
}
