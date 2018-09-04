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
    fn load(&self, h_dir: HGLOBAL, l_dir: usize) -> ShioriResult<()> {
        let mut locked = self.shiori.lock()?;
        *locked = None;
        let g_dir = GStr::capture(h_dir, l_dir);
        let dir = g_dir.to_ansi_str()?;
        let shiori = TS::load(self.get_h_inst(), dir)?;
        *locked = Some(shiori);
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
        let mut locked = self.shiori.lock()?;
        *locked = None;
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
        let len_req = len.clone();
        let g_req = GStr::capture(h, len_req);
        let req = g_req.to_utf8_str()?;
        let res = {
            let mut locked = self.shiori.lock()?;
            let shiori = locked.as_mut().ok_or(ErrorKind::NotInitialized)?;
            shiori.request(req)?
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use api::*;
    use error::*;
    use std::path::Path;

    #[derive(Default)]
    struct TestShiori {
        h_inst: usize,
    }
    impl Drop for TestShiori {
        fn drop(&mut self) {}
    }
    impl Shiori3 for TestShiori {
        fn load<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> ShioriResult<Self> {
            Ok(Default::default())
        }
        fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> ShioriResult<&'a str> {
            Ok("OK")
        }
    }

    #[test]
    fn init_test() {
        let api: RawAPI<TestShiori> = Default::default();
        {
            api.raw_shiori3_dll_main(123, DLL_PROCESS_ATTACH, ptr::null_mut());
            assert_eq!(api.get_h_inst(), 123);
            let locked = api.shiori.lock().unwrap();
            assert!(locked.is_none());
        }
        {}
        {
            api.raw_shiori3_dll_main(456, DLL_PROCESS_DETACH, ptr::null_mut());
            assert_eq!(api.get_h_inst(), 123);
            let locked = api.shiori.lock().unwrap();
            assert!(locked.is_none());
        }
    }
}
