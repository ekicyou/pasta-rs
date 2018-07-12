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

pub trait RawShiori3 {
    fn shiori3_load(hdir: HGLOBAL, len: usize) -> bool {
        false
    }

    fn shiori3_unload() -> bool {
        false
    }

    fn shiori3_request(h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        ptr::null_mut()
    }

    fn shiori3_dll_main(h_inst: usize, ul_reason_for_call: DWORD, lpReserved: LPVOID) -> bool {
        false
    }
}
