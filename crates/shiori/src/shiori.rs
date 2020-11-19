use crate::ghost::Ghost;
use once_cell::sync::Lazy;
use shiori3::*;
use std::convert::TryInto;
use std::sync::Mutex;

pub static SHIORI: Lazy<Mutex<PastaShiori>> = Lazy::new(|| Default::default());

#[derive(Default, Debug)]
pub struct PastaShiori(Option<Ghost>);

impl ShioriAPI for PastaShiori {
    fn load(&mut self, hinst: usize, load_dir: GPath) -> ApiResult<()> {
        self.0 = Some(Ghost::new(hinst, load_dir.try_into()?)?);
        Ok(())
    }
    fn unload(&mut self) -> ApiResult<()> {
        self.0 = None;
        Ok(())
    }
    fn request(&mut self, req: GCowStr) -> ApiResult<String> {
        match &mut self.0 {
            Some(g) => g.shiori_request(req),
            _ => Err(ApiError::NotLoad),
        }
    }
}
