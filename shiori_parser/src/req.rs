use pest;
use pest::Parser;
use std::collections::HashMap;

use shiori::*;

pub type Error = pest::error::Error<Rule>;
pub use pest::error::ErrorVariant;

/// SHIORI3リクエストの解析結果を格納します。
#[derive(PartialEq, Eq, Debug)]
pub struct ShioriRequest<'a> {
    pub version: i32,
    pub method: Rule,
    pub id: Option<&'a str>,
    pub sender: Option<&'a str>,
    pub security_level: Option<&'a str>,
    pub charset: Option<&'a str>,
    pub status: Option<&'a str>,
    pub base_id: Option<&'a str>,
    pub reference: Vec<(i32, &'a str)>,
    pub dic: HashMap<String, &'a str>,
    pub key_values: Vec<(Rule, &'a str, &'a str)>,
}

mod mes {
    pub const NOT_IMPL: &str = "未実装です。";
}

impl<'a> ShioriRequest<'a> {
    pub fn parse(text: &'a str) -> Result<ShioriRequest<'a>, Error> {
        let req = ShioriParser::parse(Rule::req, text)?.next().unwrap();
        let mut req_items = req.into_inner();

        let (version, method, mut id) = {
            let header = req_items.next().unwrap();
            let mut items = header.into_inner();
            let method = items.next().unwrap().as_rule();
            let h2_or_3 = items.next().unwrap();
            let (version, id) = match h2_or_3.as_rule() {
                Rule::header2 => {
                    let mut items = h2_or_3.into_inner();
                    let id = items.next().unwrap();
                    (2, Some(id.as_str()))
                }
                Rule::header3 => (3, None),
                _ => unimplemented!(),
            };
            (version, method, id)
        };

        {
            let key_values = req_items.next().unwrap();

            let mut sender = None;
            let mut security_level = None;
            let mut charset = None;
            let mut status = None;
            let mut base_id = None;
            let mut dic = HashMap::new();
            let mut reference = Vec::new();
            let mut kv_items = Vec::new();

            for kv in key_values.into_inner() {
                let mut items = kv.into_inner();
                let k = items.next().unwrap();
                let v = items.next().unwrap();
                let rule = k.as_rule();
                let key = v.as_str();
                let value = v.as_str();
                kv_items.push((rule, key, value));
                dic.entry(key.into()).or_insert(value);
                match rule {
                    Rule::key_id => id = Some(value),
                    Rule::key_base_id => base_id = Some(value),
                    Rule::key_status => status = Some(value),
                    Rule::key_sender => sender = Some(value),
                    Rule::key_security_level => security_level = Some(value),
                    Rule::key_charset => charset = Some(value),
                    Rule::key_ref => {
                        let mut it = k.into_inner();
                        let num = it.next().unwrap().as_str().parse().unwrap();
                        reference.push((num, value));
                    }
                    _ => (),
                }
            }
            let reference = reference.as_mut_slice();
            reference.sort_by_key(|a| a.0);

            Ok(ShioriRequest {
                version: version,
                method: method,
                id: id,
                sender: sender,
                security_level: security_level,
                charset: charset,
                status: status,
                base_id: base_id,
                dic: dic,
                key_values: kv_items,
                reference: Vec::from(reference),
            })
        }
    }
}
