use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use shiori3::enums::Token;
use shiori3::req::ShioriRequest;
use shiori3::res::ShioriResponse;

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub enum ShioriError {
    NotLoad,
}



/// SHIORI manage API
pub trait ShioriAPI<TSHIORI> {
    fn new(hinst: usize) -> TSHIORI;
    fn load<STR: AsRef<OsStr> + ?Sized>(&mut self, dir: &STR) -> Result<(), ShioriError>;
    fn unload(&mut self) -> Result<(), ShioriError>;
    fn request(&mut self, req_text: &str) -> Result<Cow<str>, ShioriError>;
}

/// SHIORI構造体
#[derive(Debug)]
pub struct Shiori {
    hinst: usize,
    load_dir: PathBuf,
}

impl Drop for Shiori {
    fn drop(&mut self) {}
}

impl Shiori {
    fn request_impl<'b, 'c>(&mut self, req_text: &'b str) -> Result<Cow<'c, str>, &'c str> {
        match ShioriRequest::from_str(req_text) {
            Err(reason) => return Ok(ShioriResponse::bad_request(reason)),
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


impl ShioriAPI<Shiori> for Shiori {
    fn new(hinst: usize) -> Shiori {
        Shiori {
            hinst: hinst,
            load_dir: PathBuf::new(),
        }
    }
    fn load<STR: AsRef<OsStr> + ?Sized>(&mut self, dir: &STR) -> Result<(), ShioriError> {
        self.load_dir = Path::new(dir).to_path_buf();
        Ok(())
    }

    fn unload(&mut self) -> Result<(), ShioriError> {
        Ok(())
    }

    fn request(&mut self, req_text: &str) -> Result<Cow<str>, ShioriError> {
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
            "X-PASTA-ERROR-REASON: unknown header\r\n",
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