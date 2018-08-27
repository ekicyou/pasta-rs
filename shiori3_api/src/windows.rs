use super::api::Shiori3;
use shiori_hglobal::GStr;
use std::marker::PhantomData;
use winapi::_core::ptr;
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
    phantom: PhantomData<TS>,
}

pub trait RawShiori3: Shiori3 {
    fn raw_shiori3_load(hdir: HGLOBAL, len: usize) -> bool {
        let g_dir = GStr::capture(hdir, len);
        false
    }

    fn raw_shiori3_unload() -> bool {
        false
    }

    fn raw_shiori3_request(h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        let len_req = len.clone();
        let g_req = GStr::capture(h, len_req);
        ptr::null_mut()
    }

    fn raw_shiori3_dll_main(h_inst: usize, ul_reason_for_call: DWORD, lpReserved: LPVOID) -> bool {
        false
    }
}
