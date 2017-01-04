use winapi::{HGLOBAL, size_t};

mod app {
    use std::sync::{RwLock, PoisonError};
    use shiori3::api::*;
    use winapi::{HGLOBAL, size_t};
    use gstr::*;
    use std::str::Utf8Error;

    #[derive(Debug)]
    pub enum AppError {
        PoisonError,
        NotLoad,
        NotRequest,
        Utf8Error(Utf8Error),
    }

    impl<T> From<PoisonError<T>> for AppError {
        fn from(_: PoisonError<T>) -> AppError {
            AppError::PoisonError
        }
    }
    impl From<Utf8Error> for AppError {
        fn from(err: Utf8Error) -> AppError {
            AppError::Utf8Error(err)
        }
    }

    lazy_static! {
         static ref PASTA: RwLock<Option<Shiori>>=RwLock::new(Option::None);
    }

    #[inline]
    pub fn load(hdir: HGLOBAL, len: size_t) -> Result<(), AppError> {
        let g = GStr::new(hdir, len);
        let os_dir = g.to_os_str().unwrap();
        let mut pasta = PASTA.write()?;
        *pasta = Shiori::load(&os_dir);
        match *pasta {
            Some(_) => Ok(()),
            _ => Err(AppError::NotLoad),
        }
    }

    #[inline]
    pub fn unload() -> Result<(), AppError> {
        let mut pasta = PASTA.write()?;
        match *pasta {
            None => return Err(AppError::NotLoad),
            Some(ref mut api) => {
                api.unload();
            }
        }
        *pasta = None;
        Ok(())
    }

    #[inline]
    pub fn request(h: &mut HGLOBAL, len: &mut size_t) -> Result<(), AppError> {
        let g = GStr::new(*h, *len);
        let req = g.to_str()?;
        let mut pasta = PASTA.write()?;
        match *pasta {
            None => return Err(AppError::NotLoad),
            Some(ref mut api) => {
                match api.request(req) {
                    None => Err(AppError::NotRequest),
                    Some(res) => {
                        let b_res = res.as_bytes();
                        let g_res = GStr::clone_from_slice_nofree(b_res);
                        *h = g_res.handle();
                        *len = g_res.len();
                        Ok(())
                    }
                }
            }
        }
    }
}



#[no_mangle]
pub extern "C" fn load(hdir: HGLOBAL, len: size_t) -> bool {
    app::load(hdir, len).is_ok()
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    app::unload().is_ok()
}

#[no_mangle]
pub extern "C" fn request(h: &mut HGLOBAL, len: &mut size_t) -> bool {
    app::request(h, len).is_ok()
}


#[test]
fn ffi_test() {
    use gstr::*;
    {
        let dir = "dir/dir";
        let g_dir = GStr::clone_from_slice_nofree(dir.as_bytes());
        let h = g_dir.handle();
        let len = g_dir.len();
        assert_eq!(7, len);
        assert!(load(h, len));
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
        let g = GStr::clone_from_slice_nofree(req.as_bytes());
        let mut h = g.handle();
        let mut len = g.len();
        assert!(request(&mut h, &mut len));
        let gres = GStr::new(h, len);
        let res = gres.to_str().unwrap();
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
        let g = GStr::clone_from_slice_nofree(req.as_bytes());
        let mut h = g.handle();
        let mut len = g.len();
        assert!(request(&mut h, &mut len));
        let gres = GStr::new(h, len);
        let res = gres.to_str().unwrap();
        assert_eq!(check, res);
    }
    {
        assert!(unload());
    }
}
