use crate::shiori::SHIORI;
use shiori3::entry;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HGLOBAL;
use winapi::shared::minwindef::LPVOID;

#[no_mangle]
pub extern "C" fn load(h_dir: HGLOBAL, len: usize) -> bool {
    entry::load(&SHIORI, h_dir, len)
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    entry::unload(&SHIORI)
}

#[no_mangle]
pub extern "C" fn request(h: HGLOBAL, len: &mut usize) -> HGLOBAL {
    entry::request(&SHIORI, h, len)
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    h_inst: usize,
    ul_reason_for_call: DWORD,
    lp_reserved: LPVOID,
) -> bool {
    match ul_reason_for_call {
        entry::DLL_PROCESS_DETACH => {
            unload();
        }
        _ => {}
    };
    entry::dllmain(h_inst, ul_reason_for_call, lp_reserved)
}
