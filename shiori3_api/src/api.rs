use super::error::ShioriResult;
use std::path::Path;

pub trait Shiori3: Drop + Default {
    fn load<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> ShioriResult<Self>;
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> ShioriResult<&'a str>;
}
