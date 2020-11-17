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

    #[error("EvalAltResult {source}")]
    RhaiEval {
        #[from]
        source: Box<EvalAltResult>,
    },

    #[error("futures error {source}")]
    FuturesSpawn {
        #[from]
        source: futures::task::SpawnError,
    },

    #[error("futures error {source}")]
    FuturesSend {
        #[from]
        source: futures::channel::mpsc::SendError,
    },

    #[error("error: {0}")]
    String(ImmutableString),
}

impl From<String> for PastaError {
    fn from(e: String) -> Self {
        PastaError::String(e.into())
    }
}

impl From<ImmutableString> for PastaError {
    fn from(e: ImmutableString) -> Self {
        PastaError::String(e.into())
    }
}

impl From<&str> for PastaError {
    fn from(e: &str) -> Self {
        PastaError::String(e.into())
    }
}

impl From<PastaError> for Box<EvalAltResult> {
    fn from(e: PastaError) -> Self {
        match e {
            PastaError::RhaiEval { source } => source,
            _ => {
                let mes = format!("{}", e);
                Box::new(EvalAltResult::ErrorRuntime(mes, Position::none()))
            }
        }
    }
}

impl From<PastaError> for String {
    fn from(e: PastaError) -> Self {
        match e {
            PastaError::String(s) => s.into(),
            _ => format!("{}", e),
        }
    }
}

impl From<PastaError> for ImmutableString {
    fn from(e: PastaError) -> Self {
        match e {
            PastaError::String(s) => s,
            _ => format!("{}", e).into(),
        }
    }
}
