use thiserror::Error;

pub type PastaResult<T> = Result<T, PastaError>;

#[derive(Error, Debug)]
pub enum PastaError {
    #[error("format error {source}")]
    Format {
        #[from]
        source: std::fmt::Error,
    },
}
