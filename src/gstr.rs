use libc::{memcpy, c_void};
use winapi::{HGLOBAL, UINT, size_t};
use kernel32::{GlobalFree, GlobalAlloc};

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
        let len = bytes.len() as size_t;
        unsafe {
            let p = bytes as *const c_void;
            let h = GlobalAlloc(GMEM_FIXED, len);
            GStr {
                h: h,
                len: len,
                has_free: false,
            }
        }
    }

    //  fn to_bytes(&self) -> &[u8] {}
}