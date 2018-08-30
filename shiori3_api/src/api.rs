use super::error::Error;
use std::path::Path;

pub trait Shiori3: Drop {
    fn new() -> Self;
    fn load<P: AsRef<Path>>(&mut self, h_inst: usize, load_dir: P) -> Result<(), Error>;
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<&'a str, Error>;
}
