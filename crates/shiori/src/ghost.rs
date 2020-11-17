use shiori3::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Ghost {
    hinst: usize,
    load_dir: PathBuf,
}

impl Drop for Ghost {
    fn drop(&mut self) {}
}

impl Ghost {
    pub fn new(hinst: usize, load_dir: PathBuf) -> Ghost {
        Self {
            hinst: hinst,
            load_dir: load_dir,
        }
    }

    pub fn shiori_request(&mut self, req: GCowStr) -> ApiResult<String> {
        let parse = ShioriRequest::parse(&req)?;
        unimplemented!();
    }
}
