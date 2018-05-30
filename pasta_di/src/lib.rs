use std::borrow::Cow;
use std::ffi::OsStr;

/// SHIORI Error
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub enum ShioriError {
    NotLoad,
}

/// SHIORI manage API
pub trait ShioriAPI {
    fn load<STR: AsRef<OsStr> + ?Sized>(&mut self, dir: &STR) -> Result<(), ShioriError>;
    fn unload(&mut self) -> Result<(), ShioriError>;
    fn request(&mut self, req_text: &str) -> Result<Cow<str>, ShioriError>;
}

/// have SHIORI
pub trait HaveShioriAPI {
  type ShioriAPI: ShioriAPI;
  fn shiori(&self) -> Self::ShioriAPI;
}