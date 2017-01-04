use winapi::{HGLOBAL, UINT, size_t};

mod app {
    use std::sync::{RwLock, PoisonError, TryLockError};
    use std::ffi::{OsStr, OsString};
    use shiori3::api::*;
    use winapi::{HGLOBAL, UINT, size_t};
    use gstr::*;
    use std::str::Utf8Error;

    // http://rust-lang-ja.org/rust-by-example/error/reenter_try.html
    #[derive(Debug)]
    pub enum AppError {
        PoisonError,
        NotLoad,
        Utf8Error(Utf8Error),
        Others,
    }

    impl<T> From<PoisonError<T>> for AppError {
        fn from(err: PoisonError<T>) -> AppError {
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
            Some(_) => {
                *pasta = None;
                Ok(())
            }
            _ => Err(AppError::NotLoad),
        }
    }

    #[inline]
    pub fn request(h: &mut HGLOBAL, len: &mut size_t) -> Result<(), AppError> {
        let g = GStr::new(*h, *len);
        let req = g.to_str()?;
        Ok(())
    }


}



#[no_mangle]
pub extern "C" fn load(hdir: HGLOBAL, len: size_t) -> bool {
    match app::load(hdir, len) {
        Ok(_) => true,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    match app::unload() {
        Ok(_) => true,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn request(h: &mut HGLOBAL, len: &mut size_t) -> bool {
    match app::request(h, len) {
        Ok(_) => true,
        _ => false,
    }
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
        assert!(unload());
    }
}
