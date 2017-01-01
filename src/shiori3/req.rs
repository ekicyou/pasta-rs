use nom::IResult;
use shiori3::enums::Token;
use shiori3::parsers::{header2, header3, kv};
use std::collections::HashMap;


/// SHIORI3リクエストの解析結果を格納します。
///
/// # Examples
///
/// ```
/// use shiori3::parsers::Token;
/// use shiori3::req::ShioriRequest;
///
/// let s = "GET SHIORI/3.0\r\nCharset: UTF-8\r\n\r\n";
/// let x = ShioriRequest::from_str(s).unwrap();
/// assert_eq!(x.version, Token::SHIORI3);
/// assert_eq!(x.method, Token::GET);
/// assert_eq!(x.charset, "UTF-8");
/// ```
#[derive(PartialEq,Eq,Debug)]
pub struct ShioriRequest<'a> {
    pub version: Token<'a>,
    pub method: Token<'a>,
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
    pub fn from_str<'b>(text: &'a str) -> Result<ShioriRequest<'a>, &'b str> {
        let mut target = &text[..];
        let mut rc = ShioriRequest {
            version: Token::NONE,
            method: Token::NONE,
            id: "",
            reference: Vec::new(),
            dic: HashMap::new(),
            sender: "",
            security_level: "",
            charset: "",
            status: "",
            base_id: "",
        };
        match header3(target) {
            IResult::Done(remain, (v, m)) => {
                target = remain;
                rc.version = v;
                rc.method = m;
            }
            _ => {
                match header2(target) {
                    IResult::Done(remain, (v, m, id)) => {
                        target = remain;
                        rc.version = v;
                        rc.method = m;
                        rc.id = id;
                    }
                    _ => return Result::Err("unknown header"),
                }
            }
        }
        loop {
            match kv(target) {
                IResult::Done(remain, Token::KV(k, v)) => {
                    target = remain;
                    if k.starts_with("Reference") {
                        rc.reference.push(v);
                    } else if k == "ID" {
                        rc.id = v;
                    } else if k == "Charset" {
                        rc.charset = v;
                    } else if k == "Sender" {
                        rc.sender = v;
                    } else if k == "SecurityLevel" {
                        rc.security_level = v;
                    } else if k == "Status" {
                        rc.status = v;
                    } else if k == "BaseID" {
                        rc.base_id = v;
                    } else {
                        rc.dic.insert(k.to_string(), v);
                    }
                }
                _ => break,
            }
        }
        Result::Ok(rc)
    }
}


#[test]
fn req_parser_test() {
    {
        let text = concat!(
            "GET Version SHIORI/2.6\r\n",
            "Charset: UTF-8\r\n",
            "Sender: SSP\r\n",
            "\r\n",
        );
        let req = ShioriRequest::from_str(text).unwrap();
        assert_eq!(req.version, Token::SHIORI2("SHIORI/2.6"));
        assert_eq!(req.method, Token::GET);
        assert_eq!(req.charset, "UTF-8");
        assert_eq!(req.sender, "SSP");
        assert_eq!(req.id, "Version");
    }
    {
        let text = concat!(
            "GET SHIORI/3.0\r\n",
            "Charset: UTF-8\r\n",
            "Sender: SSP\r\n",
            "SecurityLevel: local\r\n",
            "Status: balloon(0=0)\r\n",
            "ID: OnMouseDoubleClick\r\n",
            "BaseID: OnMouseDown\r\n",
            "Reference0: 139\r\n",
            "Reference1: 42\r\n",
            "Reference2: 0\r\n",
            "Reference3: 0\r\n",
            "Reference4: \r\n",
            "Reference5: 0\r\n",
            "Reference6: mouse\r\n",
            "\r\n",
        );
        let req = ShioriRequest::from_str(text).unwrap();
        assert_eq!(req.version, Token::SHIORI3);
        assert_eq!(req.method, Token::GET);
        assert_eq!(req.charset, "UTF-8");
        assert_eq!(req.sender, "SSP");
        assert_eq!(req.security_level, "local");
        assert_eq!(req.status, "balloon(0=0)");
        assert_eq!(req.id, "OnMouseDoubleClick");
        assert_eq!(req.base_id, "OnMouseDown");
        assert_eq!(req.reference.len(), 7);
        assert_eq!(req.reference[0], "139");
        assert_eq!(req.reference[1], "42");
        assert_eq!(req.reference[2], "0");
        assert_eq!(req.reference[3], "0");
        assert_eq!(req.reference[4], "");
        assert_eq!(req.reference[5], "0");
        assert_eq!(req.reference[6], "mouse");
    }
}
