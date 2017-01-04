use winapi::{HGLOBAL, UINT, size_t};

mod app {
    use std::sync::{RwLock, TryLockError};
    use std::ffi::{OsStr, OsString};
    use shiori3::api::*;
    use winapi::{HGLOBAL, UINT, size_t};
    use gstr::*;

    // http://rust-lang-ja.org/rust-by-example/error/reenter_try.html
    #[derive(Debug)]
    pub enum AppError<T> {
        TryLock(TryLockError<T>),
        Others,
    }

    impl<T> From<TryLockError<T>> for AppError<T> {
        fn from(err: TryLockError<T>) -> AppError<T> {
            AppError::TryLock(err)
        }
    }

    lazy_static! {
         static ref PASTA: RwLock<Option<Shiori>>=RwLock::new(Option::None);
    }

    #[inline]
    pub fn load(hdir: HGLOBAL, len: size_t) -> Result<(), String> {
        let g = GStr::new(hdir, len);
        let os_dir = g.to_os_str().unwrap();
        let mut pasta = match PASTA.write() {
            Ok(a) => a,
            _ => return Err("rw_lock_poison".to_string()),
        };
        *pasta = Shiori::load(&os_dir);
        match *pasta {
            Some(_) => Ok(()),
            _ => Err("load_error".to_string()),
        }
    }

    #[inline]
    pub fn unload() -> Result<(), String> {
        Ok(())
    }

    #[inline]
    pub fn request(h: &mut HGLOBAL, len: &mut size_t) -> Result<(), String> {
        let g = GStr::new(*h, *len);
        let req = match g.to_str() {
            Ok(a) => a,
            _ => return Err("not utf-8".to_string()),
        };
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
