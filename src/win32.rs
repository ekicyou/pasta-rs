use std::ffi::*;
use winapi::*;

#[no_mangle]
pub unsafe extern "C" fn load(hdir: HGLOBAL, len: size_t) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn unload() -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn request(hreq: HGLOBAL, len: &mut size_t) -> bool {
    true
}



#[test]
fn ffi_test() {
    unsafe {
    }
}
