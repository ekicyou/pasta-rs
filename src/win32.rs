use winapi::{HGLOBAL, UINT, size_t};
use std::ffi::OsStr;
use gstr::*;
use shiori3::api::*;
use std::cell::RefCell;

const GMEM_FIXED: UINT = 0;

// static mut pasta: Option<RefCell<Shiori>> = None;


#[no_mangle]
pub extern "C" fn load(hdir: HGLOBAL, len: size_t) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn request(hreq: HGLOBAL, len: &mut size_t) -> bool {
    true
}


#[test]
fn ffi_test() {
    {
        let dir = "dir/dir";
        let g_dir = GStr::clone_from_slice_free(dir.as_bytes());
        let os_dir = g_dir.to_os_str().unwrap();
        let h = g_dir.handle();
        let len = g_dir.len();
        assert_eq!(7, len);
        assert!(load(h, len));
    }
    {
        assert!(unload());
    }
}
