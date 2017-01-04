use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use shiori3::enums::Token;
use shiori3::req::ShioriRequest;
use shiori3::res::ShioriResponse;

/// SHIORI manage API
pub trait ShioriAPI {
    fn load<STR: AsRef<OsStr> + ?Sized>(module: &usize, dir: &STR) -> Option<Shiori>;
    fn unload(&mut self) -> bool;
    fn request(&mut self, req_text: &str) -> Option<Cow<str>>;
}

/// SHIORI構造体
#[derive(Debug)]
pub struct Shiori {
    module: usize,
    load_dir: PathBuf,
}

impl Drop for Shiori {
    fn drop(&mut self) {}
}

impl Shiori {
    fn request_impl<'b, 'c>(&mut self, req_text: &'b str) -> Result<Cow<'c, str>, &'c str> {
        match ShioriRequest::from_str(req_text) {
            Err(reason) => return Result::Ok(ShioriResponse::bad_request(reason)),
            Ok(req) => {
                if req.id == "OnBoot" {
                    let talk = r"\1\s[10]\0\s[0]やあ、元気？\e";
                    return Result::Ok(ShioriResponse::talk(talk));
                }
            }
        }
        Result::Ok(ShioriResponse::no_content())
    }
}


impl ShioriAPI for Shiori {
    fn load<STR: AsRef<OsStr> + ?Sized>(module: &usize, dir: &STR) -> Option<Shiori> {
        let shiori = Shiori {
            module: *module,
            load_dir: Path::new(dir).to_path_buf(),
        };
        Option::Some(shiori)
    }

    fn unload(&mut self) -> bool {
        true
    }

    fn request(&mut self, req_text: &str) -> Option<Cow<str>> {
        let res = self.request_impl(req_text);
        let rc = match res {
            Ok(value) => value,
            Err(reason) => ShioriResponse::internal_server_error(reason),
        };
        Option::Some(rc)
    }
}

#[test]
fn shiori_test() {
    let dir_data = "LOAD_DIR";
    let rc = Shiori::load(&0, dir_data);
    let mut shiori = rc.unwrap();
    assert!(shiori.load_dir.to_str().unwrap() == dir_data);
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
        assert!(shiori.unload() == true);
    }
}