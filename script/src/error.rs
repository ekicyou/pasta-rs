use rhai::{EvalAltResult, ImmutableString, Position};
use std::convert::From;
use thiserror::Error;

pub type PastaResult<T> = Result<T, PastaError>;

#[derive(Error, Debug)]
pub enum PastaError {
    #[error("not found actor_name {0}")]
    ActorNotFound(ImmutableString),

    #[error("format error {source}")]
    Format {
        #[from]
        source: std::fmt::Error,
    },

    #[error("io error {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
}

impl From<PastaError> for EvalAltResult {
    fn from(e: PastaError) -> Self {
        let mes = format!("{}", e);
        EvalAltResult::ErrorRuntime(mes, Position::none())
    }
}

impl From<PastaError> for String {
    fn from(e: PastaError) -> Self {
        format!("{}", e)
    }
}

impl From<PastaError> for ImmutableString {
    fn from(e: PastaError) -> Self {
        format!("{}", e).into()
    }
}
