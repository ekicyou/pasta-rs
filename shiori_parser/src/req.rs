use std::collections::HashMap;
use super::shiori::*;
use pest::Parser;

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub enum ReqParseError {
    UnknownHeader,
}

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub enum Method {
    Get,
    Notify,
}

/// SHIORI3リクエストの解析結果を格納します。
#[derive(PartialEq,Eq,Debug)]
pub struct ShioriRequest<'a> {
    pub version: i32,
    pub method: Method,
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
    pub fn from_str(text: &'a str) -> Result<ShioriRequest<'a>, ReqParseError> {
        

        Err(ReqParseError::UnknownHeader)
    }
}
