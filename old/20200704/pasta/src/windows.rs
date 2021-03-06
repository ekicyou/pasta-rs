use winapi::{HGLOBAL, DWORD, LPVOID};

mod app {
    use std::sync::{RwLock, PoisonError};
    use shiori3::api::*;
    use std::*;
    use winapi::HGLOBAL;
    use std::str::Utf8Error;
    use shiori_hglobal::*;

    #[derive(Copy, Eq, PartialEq, Clone, Debug)]
    pub enum AppError {
        PoisonError,
        NotLoad,
        ShioriError(ShioriError),
        Utf8Error(Utf8Error),
        GStrError(GStrError),
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
    impl From<ShioriError> for AppError {
        fn from(err: ShioriError) -> AppError {
            AppError::ShioriError(err)
        }
    }
    impl From<GStrError> for AppError {
        fn from(err: GStrError) -> AppError {
            AppError::GStrError(err)
        }
    }


    lazy_static! {
        static ref PASTA: RwLock<Result<Shiori,AppError>> = RwLock::new(Err(AppError::NotLoad));
    }


    #[inline]
    pub fn init(hInst: usize) -> Result<(), AppError> {
        let mut pasta = PASTA.write()?;
        *pasta = Ok(Shiori::new(hInst));
        Ok(())
    }

    #[inline]
    pub fn load(hdir: HGLOBAL, len: usize) -> Result<(), AppError> {
        let g = GStr::capture(hdir, len);
        let os_dir = g.to_load_str()?;
        let mut pasta = PASTA.write()?;
        match *pasta {
            Err(e) => Err(e),
            Ok(ref mut api) => {
                api.load(&os_dir)?;
                Ok(())
            }
        }
    }

    #[inline]
    pub fn unload() -> Result<(), AppError> {
        let mut pasta = PASTA.write()?;
        match *pasta {
            Err(e) => return Err(e),
            Ok(ref mut api) => {
                api.unload()?;
            }
        }
        *pasta = Err(AppError::NotLoad);
        Ok(())
    }

    #[inline]
    pub fn request(h: HGLOBAL, len: &mut usize) -> Result<HGLOBAL, AppError> {
        let g = GStr::capture(h, *len);
        let req = g.to_req_str()?;
        let mut pasta = PASTA.write()?;
        match *pasta {
            Err(e) => Err(e),
            Ok(ref mut api) => {
                let res = api.request(req)?;
                let b_res = res.as_bytes();
                let g_res = GStr::clone_from_slice_nofree(b_res);
                *len = g_res.len();
                Ok(g_res.handle())
            }
        }
    }
}



#[no_mangle]
pub extern "C" fn load(hdir: HGLOBAL, len: usize) -> bool {
    app::load(hdir, len).is_ok()
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    app::unload().is_ok()
}

#[no_mangle]
pub extern "C" fn request(h: HGLOBAL, len: &mut usize) -> HGLOBAL {
    match app::request(h, len) {
        Ok(res) => res,
        Err(_) => ::std::ptr::null_mut(),
    }
}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

#[no_mangle]
#[allow(dead_code,unused_must_use,unused_variables)]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hInst: usize,
                                ul_reason_for_call: DWORD,
                                lpReserved: LPVOID)
                                -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            return app::init(hInst).is_ok();
        }
        DLL_PROCESS_DETACH => {
            app::unload();
        }
        _ => {}
    }
    true
}


#[test]
fn ffi_test() {
    use shiori_hglobal::*;
    {
        ::std::env::set_var("RUST_LOG", "trace");
        let _ = ::env_logger::init();
    }
    {
        assert_eq!(unload(), false);
        assert!(app::init(0).is_ok());
    }
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
        let h = g.handle();
        let mut len = g.len();
        let hres = request(h, &mut len);
        let gres = GStr::capture(hres, len);
        let res = gres.to_req_str().unwrap();
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
        let h = g.handle();
        let mut len = g.len();
        let hres = request(h, &mut len);
        let gres = GStr::capture(hres, len);
        let res = gres.to_req_str().unwrap();
        assert_eq!(check, res);
    }
    {
        trace!("unload start");
        assert_eq!(unload(), true);
        trace!("unload end");
        assert_eq!(unload(), false);
    }
}
