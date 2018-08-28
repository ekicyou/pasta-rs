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
    phantom: PhantomData<TS>,
}

impl<TS: Shiori3> RawAPI<TS> {
    pub fn new() -> Self {
        Self {
            inst: Mutex::default(),
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
        let g_dir = GStr::capture(hdir, len);
        let mut shiori = self.inst.lock()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_unload(&mut self) -> bool {
        false
    }
    #[allow(dead_code)]
    pub fn raw_shiori3_request(&mut self, h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        let len_req = len.clone();
        let g_req = GStr::capture(h, len_req);
        ptr::null_mut()
    }
    #[allow(dead_code)]
    pub fn raw_shiori3_dll_main(
        &mut self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        lpReserved: LPVOID,
    ) -> bool {
        true
    }
}
