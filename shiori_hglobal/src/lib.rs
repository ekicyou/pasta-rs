#![cfg(any(windows))]

extern crate winapi;
extern crate kernel32;
extern crate local_encoding;


use std::mem::transmute;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::ffi::OsString;
use std::str;
use std::str::Utf8Error;
use winapi::{HGLOBAL, UINT, size_t};
use kernel32::{GlobalFree, GlobalAlloc};
use local_encoding::{Encoding, Encoder};

const GMEM_FIXED: UINT = 0;

#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub enum GStrError {
    AnsiEncode,
    Utf8Error(Utf8Error),
}
impl From<Utf8Error> for GStrError {
    fn from(err: Utf8Error) -> GStrError {
        GStrError::Utf8Error(err)
    }
}

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
    /// shiori::load/requestのHGLOBAL受け入れに利用してください。
    pub fn capture(h: HGLOBAL, len: usize) -> GStr {
        GStr {
            h: h,
            len: len,
            has_free: true,
        }
    }

    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice_impl(bytes: &[u8], has_free: bool) -> GStr {
        let len = bytes.len();
        unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let p = transmute::<HGLOBAL, *mut u8>(h);
            let dst = from_raw_parts_mut::<u8>(p, len);
            dst[..].clone_from_slice(bytes);
            GStr {
                h: h,
                len: len,
                has_free: has_free,
            }
        }
    }

    /// HGLOBALを新たに作成し、&[u8]をGStrにクローンします。
    /// drop時にHGLOBALを開放しません。
    /// shiori応答の作成に利用してください。
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
    pub fn len(&self) -> usize {
        self.len
    }

    /// 格納データを「ANSI STRING(JP環境ではSJIS)」とみなして、OsStrに変換する。
    /// MultiByteToWideChar()を利用する。
    /// SHIORI::load()文字列の取り出しに利用する。
    pub fn to_load_str(&self) -> Result<OsString, GStrError> {
        let bytes = self.to_bytes();
        let s = Encoding::ANSI.to_string(bytes).map_err(|_| GStrError::AnsiEncode)?;
        let os_str = OsString::from(s);
        Ok(os_str)
    }

    /// 格納データを「UTF-8」とみなして、strに変換する。
    /// SHIORI::request()文字列の取り出しに利用する。
    pub fn to_req_str(&self) -> Result<&str, GStrError> {
        let bytes = self.to_bytes();
        Ok(str::from_utf8(bytes)?)
    }
}

#[test]
fn gstr_test() {
    {
        let text = "適当なGSTR";
        let src = GStr::clone_from_slice_nofree(text.as_bytes());
        assert_eq!(src.to_req_str().unwrap(), text);
        assert_eq!(src.len(), 13);

        let dst = GStr::capture(src.handle(), src.len());
        assert_eq!(dst.to_req_str().unwrap(), text);
    }
    {
        let text = "適当なGSTR";
        let sjis = Encoding::ANSI.to_bytes(text).unwrap();
        assert_eq!(sjis.len(), 10);
        let src = GStr::clone_from_slice_nofree(&sjis[..]);
        assert_eq!(src.len(), 10);
        let src_osstr = src.to_load_str().unwrap();
        assert_eq!(src_osstr.len(), 13);

        let dst = GStr::capture(src.handle(), src.len());
        assert_eq!(src_osstr, dst.to_load_str().unwrap());

        let src_str = src_osstr.to_str().unwrap();
        assert_eq!(src_str, text);
    }
}
