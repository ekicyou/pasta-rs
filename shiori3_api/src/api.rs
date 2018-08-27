use std::path::Path;

pub trait Shiori3 {
    fn new() -> Self;
    fn load<P: AsRef<Path>>(&mut self, h_inst: usize, load_dir: P) -> Result<(), ()>;
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<&'a str, ()>;
}
