use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::sync::{Arc, Mutex};
use super::enums::Token;
use super::req::ShioriRequest;
use super::res::ShioriResponse;
use pasta_di::shiori::*;


/// SHIORI構造体
#[derive(Debug)]
pub struct Shiori {
    dao: Arc<Mutex<HaveShioriAPI>>,
    hinst: usize,
    load_dir: PathBuf,
}

impl Shiori {
    fn new(hinst: usize, dao: Arc<Mutex<HaveShioriAPI>>) -> Shiori {
        trace!("Shiori::new");
        Shiori {
            dao: dao,
            hinst: hinst,
            load_dir: PathBuf::new(),
        }
    }
    fn request_impl<'b, 'c>(&mut self, req_text: &'b str) -> Result<Cow<'c, str>, &'c str> {
        match ShioriRequest::from_str(req_text) {
            Err(reason) => {
                let text = format!("{:?}", reason);
                return Ok(ShioriResponse::bad_request(&text));
            } 
            Ok(req) => {
                if req.id == "OnBoot" {
                    let talk = r"\1\s[10]\0\s[0]やあ、元気？\e";
                    return Ok(ShioriResponse::talk(talk));
                }
            }
        }
        Ok(ShioriResponse::no_content())
    }
}

impl Drop for Shiori {
    fn drop(&mut self) {
        trace!("Shiori::drop\n{:?}", self);
    }
}


impl ShioriAPI for Shiori {
    fn load<STR: AsRef<OsStr> + ?Sized>(&mut self, dir: &STR) -> Result<(), ShioriError> {
        trace!("Shiori::load");
        self.load_dir = Path::new(dir).to_path_buf();
        Ok(())
    }

    fn unload(&mut self) -> Result<(), ShioriError> {
        trace!("Shiori::unload");
        Ok(())
    }

    fn request(&mut self, req_text: &str) -> Result<Cow<str>, ShioriError> {
        trace!("Shiori::request");
        let res = self.request_impl(req_text);
        let rc = match res {
            Ok(value) => value,
            Err(reason) => ShioriResponse::internal_server_error(reason),
        };
        Ok(rc)
    }
}

#[test]
fn shiori_test() {
    let mut shiori = Shiori::new(0);
    {
        let dir_data = "LOAD_DIR";
        assert!(shiori.load(dir_data).is_ok());
    }
    {
        let req = "REQ";
        let check = concat!(
            "SHIORI/3.0 400 Bad Request\r\n",
            "Charset: UTF-8\r\n",
            "X-PASTA-ERROR-REASON: UnknownHeader\r\n",
            "\r\n",
        );
        let res = shiori.request(req).unwrap();
        assert_eq!(check, res);
    }
    {
        let req = concat!(
            "NOTIFY SHIORI/3.0\r\n",
            "Charset: UTF-8\r\n",
            "Sender: SSP\r\n",
            "SecurityLevel: local\r\n",
            "ID: OnInitialize\r\n",
            "Reference0: \r\n",
            "\r\n",
        );
        let check = concat!(
            "SHIORI/3.0 204 No Content\r\n",
            "Charset: UTF-8\r\n",
            "\r\n",
        );
        let res = shiori.request(req).unwrap();
        assert_eq!(check, res);
    }
    {
        let req = concat!(
            "GET SHIORI/3.0\r\n",
            "Charset: UTF-8\r\n",
            "Sender: SSP\r\n",
            "SecurityLevel: local\r\n",
            "ID: OnBoot\r\n",
            "Reference0: マスターシェル\r\n",
            "\r\n",
        );
        let check = concat!(
            "SHIORI/3.0 200 OK\r\n",
            "Charset: UTF-8\r\n",
            "Value: ", r"\1\s[10]\0\s[0]やあ、元気？\e", "\r\n",
            "\r\n",
        );
        let res = shiori.request(req).unwrap();
        assert_eq!(check, res);
    }
    {
        assert!(shiori.unload().is_ok());
    }
}