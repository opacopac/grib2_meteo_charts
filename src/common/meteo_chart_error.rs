use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::imaging_error::ImagingError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoChartError {
    #[error("invalid data: {0}")]
    InvalidData(String),

    #[error(transparent)]
    ImagingError(#[from] ImagingError),

    #[error(transparent)]
    Grib2(#[from] Grib2Error),

    #[error(transparent)]
    Ureq(#[from] ureq::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Image(#[from] image::ImageError),

    // fallback for other boxed errors:
    #[error("internal error: {0}")]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}
