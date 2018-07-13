use std::path::Path;

pub trait Shiori3 {
    fn shiori3_get<'a, TS: Shiori3>() -> Option<&'a mut TS>;
    fn shiori3_load<'a, TS: Shiori3, P: AsRef<Path>>(
        h_inst: usize,
        load_dir: P,
    ) -> Option<&'a mut TS>;
    fn shiori3_drop() -> bool;

    fn shiori3_request<'a, S: Into<&'a str>>(&mut self, req: S) -> str;
}
