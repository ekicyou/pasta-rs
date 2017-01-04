use winapi::{HGLOBAL, UINT, size_t};
use std::ffi::OsStr;
use gstr::*;
use shiori3::api::*;
use std::cell::RefCell;

const GMEM_FIXED: UINT = 0;

// static mut pasta: Option<RefCell<Shiori>> = None;


#[no_mangle]
pub extern "C" fn load(hdir: HGLOBAL, len: size_t) -> bool {
    let g = GStr::new(hdir, len);
    let os_dir = g.to_os_str().unwrap();

    true
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn request(hreq: HGLOBAL, len: &mut size_t) -> bool {
    let g = GStr::new(hreq, *len);
    let req = match g.to_str() {
        Ok(s) => s,
        _ => "",
    };

    true
}


#[test]
fn ffi_test() {
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
