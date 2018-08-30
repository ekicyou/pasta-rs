use super::api::Shiori3;
use super::error::*;
use shiori_hglobal::GStr;
use std::cell::UnsafeCell;
use std::mem;
use std::ptr;
use std::sync::Mutex;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

#[derive(Default)]
struct ImplData<TS: Shiori3> {
    shiori: Option<TS>,
    h_inst: usize,
}

#[derive(Default)]
pub struct RawAPI<TS: Shiori3> {
    value: UnsafeCell<ImplData<TS>>,
}

impl<TS: Shiori3> RawAPI<TS> {
    fn get_ref(&self) -> &mut ImplData<TS> {
        let p = self.value.get();
        unsafe { mem::transmute(p) }
    }
    fn get_shiori(&self) -> ShioriResult<&mut TS> {
        let data = self.get_ref();
        Ok(data.shiori.as_mut().ok_or(ErrorKind::NotInitialized)?)
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_load(&self, hdir: HGLOBAL, len: usize) -> bool {
        match self.load(hdir, len) {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }

    fn load(&self, hdir: HGLOBAL, len: usize) -> ShioriResult<()> {
        let data = self.get_ref();
        data.shiori = None;
        let mut shiori = TS::new();
        let g_dir = GStr::capture(hdir, len);
        let dir = g_dir.to_ansi_str()?;
        shiori.load(data.h_inst, dir)?;
        data.shiori = Some(shiori);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_unload(&self) -> bool {
        match self.unload() {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }
    fn unload(&self) -> ShioriResult<()> {
        let data = self.get_ref();
        data.shiori = None;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_request(&self, h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match self.request(h, len) {
            Err(e) => {
                error!("{}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok(rc) => rc,
        }
    }
    pub fn request(&self, h: HGLOBAL, len: &mut usize) -> ShioriResult<HGLOBAL> {
        let shiori = self.get_shiori()?;
        let len_req = len.clone();
        let g_req = GStr::capture(h, len_req);
        let req = g_req.to_utf8_str()?;
        let res = shiori.request(req)?;
        let b_res = res.as_bytes();
        let g_res = GStr::clone_from_slice_nofree(b_res);
        *len = g_res.len();
        Ok(g_res.handle())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_dll_main(
        &self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                let mut data = self.get_ref();
                data.h_inst = h_inst;
            }
            DLL_PROCESS_DETACH => {
                self.raw_shiori3_unload();
            }
            _ => {}
        }
        true
    }
}
