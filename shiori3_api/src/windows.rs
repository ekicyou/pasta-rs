use super::api::Shiori3;
use super::error::*;
use shiori_hglobal::GStr;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
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
pub struct RawAPI<TS: Shiori3> {
    shiori: Mutex<Option<TS>>,
    h_inst: AtomicUsize,
}

impl<TS: Shiori3> RawAPI<TS> {
    fn get_shiori(&self) -> ShioriResult<&mut TS> {
        let locked = self.shiori.lock()?;
        let shiori = *locked.as_mut().ok_or(ErrorKind::NotInitialized)?;
        Ok(shiori)
    }
    fn set_shiori(&self, obj: Option<TS>) -> ShioriResult<()> {
        let target = self.shiori.get_mut()?;
        *target = obj;
        Ok(())
    }
    fn get_h_inst(&self) -> usize {
        self.h_inst.load(Ordering::Relaxed)
    }
    fn set_h_inst(&self, value: usize) {
        self.h_inst.store(value, Ordering::Relaxed)
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
        self.set_shiori(None)?;
        let mut shiori = TS::new();
        let g_dir = GStr::capture(hdir, len);
        let dir = g_dir.to_ansi_str()?;
        shiori.load(self.get_h_inst(), dir)?;
        self.set_shiori(Some(shiori))?;
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
        self.set_shiori(None)?;
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
                self.set_h_inst(h_inst);
            }
            DLL_PROCESS_DETACH => {
                self.raw_shiori3_unload();
            }
            _ => {}
        }
        true
    }
}
