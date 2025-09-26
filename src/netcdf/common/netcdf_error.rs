use thiserror::Error;


#[derive(Debug, Error)]
pub enum NetCdfError {
    #[error("The NetCDF data is invalid: {0}")]
    InvalidData(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
}
