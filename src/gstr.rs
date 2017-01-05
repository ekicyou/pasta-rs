use std::mem::transmute;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::ffi::OsString;
use std::str;
use std::str::Utf8Error;
use std::io::Error;
use winapi::{HGLOBAL, UINT, size_t};
use kernel32::{GlobalFree, GlobalAlloc};
use local_encoding::{Encoding, Encoder};

const GMEM_FIXED: UINT = 0;

/// HGLOBALを文字列にキャプチャーします。
pub struct GStr {
    h: HGLOBAL,
    len: usize,
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
    /// HGLOBALをGStrにキャプチャーします。
    /// drop時にHGLOBALを開放します。
    pub fn new(h: HGLOBAL, len: size_t) -> GStr {
        GStr {
            h: h,
            len: len as usize,
            has_free: true,
        }
    }

    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice_impl(bytes: &[u8], has_free: bool) -> GStr {
        let len = bytes.len();
        unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let p = transmute::<HGLOBAL, *mut u8>(h);
            let mut dst = from_raw_parts_mut::<u8>(p, len);
            dst[..].clone_from_slice(bytes);
            GStr {
                h: h,
                len: len,
                has_free: has_free,
            }
        }
    }

    /// &[u8]をGStrにキャプチャーします。
    /// drop時にHGLOBALを開放しません。
    pub fn clone_from_slice_nofree(bytes: &[u8]) -> GStr {
        GStr::clone_from_slice_impl(bytes, false)
    }

    /// 要素を&[u8]として取り出します。
    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let p = transmute::<HGLOBAL, *mut u8>(self.h);
            from_raw_parts::<u8>(p, self.len)
        }
    }

    /// HGLOBALハンドルを取得します。
    pub fn handle(&self) -> HGLOBAL {
        self.h
    }

    /// 領域サイズを取得します。
    pub fn len(&self) -> size_t {
        self.len as size_t
    }

    /// 格納データを「ANSI STRING」とみなして、OsStrに変換する。
    /// MultiByteToWideChar()を利用する。
    pub fn to_os_str(&self) -> Result<OsString, Error> {
        let bytes = self.to_bytes();
        let s = Encoding::ANSI.to_string(bytes)?;
        let os_str = OsString::from(s);
        Ok(os_str)
    }

    /// 格納データを「UTF-8」とみなして、strに変換する。
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        let bytes = self.to_bytes();
        str::from_utf8(bytes)
    }
}