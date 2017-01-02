use winapi::{HGLOBAL, UINT, size_t};
use kernel32::{GlobalFree, GlobalAlloc};

const GMEM_FIXED: UINT = 0;

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
        let dir_bytes = "dir/dir".bytes();
        let dir_len = dir_bytes.count() as size_t;


        unsafe {
            let hdir = GlobalAlloc(GMEM_FIXED, dir_len);
            let rc = load(hdir, dir_len);



        }
    }
}
