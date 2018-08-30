use super::api::Shiori3;
use super::error::*;
use shiori_hglobal::GStr;
use std::marker::PhantomData;
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

pub struct RawAPI<TS: Shiori3> {
    inst: Mutex<Option<TS>>,
    h_inst: usize,
    phantom: PhantomData<TS>,
}

impl<TS: Shiori3> RawAPI<TS> {
    pub fn new() -> Self {
        Self {
            inst: Mutex::default(),
            h_inst: 0,
            phantom: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_load(&mut self, hdir: HGLOBAL, len: usize) -> bool {
        match self.load(hdir, len) {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }
    fn load(&mut self, hdir: HGLOBAL, len: usize) -> ShioriResult<()> {
        let mut shiori_inst = self.inst.lock()?;
        *shiori_inst = None;
        let mut shiori = TS::new();
        let g_dir = GStr::capture(hdir, len);
        let dir = g_dir.to_ansi_str()?;
        shiori.load(self.h_inst, dir)?;
        *shiori_inst = Some(shiori);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_unload(&mut self) -> bool {
        match self.unload() {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }
    fn unload(&mut self) -> ShioriResult<()> {
        let mut shiori_inst = self.inst.lock()?;
        *shiori_inst = None;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_request(&mut self, h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match self.request(h, len) {
            Err(e) => {
                error!("{}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok(rc) => rc,
        }
    }
    pub fn request(&mut self, h: HGLOBAL, len: &mut usize) -> ShioriResult<HGLOBAL> {
        let mut shiori_inst = self.inst.lock()?;
        let shiori = (*shiori_inst).as_mut().ok_or(ErrorKind::NotInitialized)?;
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
        &mut self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                self.h_inst = h_inst;
            }
            DLL_PROCESS_DETACH => {
                self.raw_shiori3_unload();
            }
            _ => {}
        }
        true
    }
}
