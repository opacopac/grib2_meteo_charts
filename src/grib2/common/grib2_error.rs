use thiserror::Error;
use crate::system::system_error::SystemError;


#[derive(Debug, Error)]
pub enum Grib2Error {
    #[error("invalid data: {0}")]
    InvalidData(String),
    
    #[error(transparent)]
    SystemError(#[from] SystemError),

    #[error(transparent)]
    Ureq(#[from] ureq::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
}
