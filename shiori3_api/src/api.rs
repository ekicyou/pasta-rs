use std::path::Path;

pub trait Shiori3 {
    type Item: Shiori3;

    fn get_shiori3() -> Option<&mut Item>;
    fn load_shiori3<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> Option<&mut Item>;
    fn drop_shiori3() -> bool;
}
