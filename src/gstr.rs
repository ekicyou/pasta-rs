use libc::{memcpy, c_void};
use winapi::{HGLOBAL, UINT, size_t};
use kernel32::{GlobalFree, GlobalAlloc};
use std::mem::transmute;

const GMEM_FIXED: UINT = 0;

/// HGLOBALとして送受信される文字列
struct GStr {
    h: HGLOBAL,
    len: size_t,
    has_free: bool,
}

impl Drop for GStr {
    fn drop(&mut self) {
        if !self.has_free {
            return;
        }
        unsafe {
            GlobalFree(self.h);
        }
    }
}

impl GStr {
    fn new(h: HGLOBAL, len: size_t) -> GStr {
        GStr {
            h: h,
            len: len,
            has_free: true,
        }
    }
    fn from_bytes(bytes: &[u8]) -> GStr {
        let len = bytes.len();
        unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let mut dst = transmute::<HGLOBAL, [u8; len]>(h);

            GStr {
                h: h,
                len: len,
                has_free: false,
            }
        }
    }

    //  fn to_bytes(&self) -> &[u8] {}
}