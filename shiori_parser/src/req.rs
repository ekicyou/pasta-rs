use pest;
use std::collections::HashMap;

use shiori::*;

pub type Error = pest::error::Error<Rule>;
pub use pest::error::ErrorVariant;

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

mod mes{
    pub const NOT_IMPL:&str = "未実装です。";
}


impl<'a> ShioriRequest<'a> {
    pub fn from_str(text: &'a str) -> Result<ShioriRequest<'a>, Error> {
        Err(Error::new_from_pos(
            ErrorVariant::CustomError{
                message: mes::NOT_IMPL.to_string(),
            },
            pest::Position::from_start(text)
        ))
    }
}
