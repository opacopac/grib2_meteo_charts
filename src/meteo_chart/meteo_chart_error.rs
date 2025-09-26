use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::imaging_error::ImagingError;
use crate::map_tile::map_tile_error::MapTileError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoChartError {
    #[error("invalid data: {0}")]
    InvalidData(String),

    #[error(transparent)]
    Grib2(#[from] Grib2Error),

    #[error(transparent)]
    ImagingError(#[from] ImagingError),

    #[error(transparent)]
    MapTileError(#[from] MapTileError),

    /*#[error(transparent)]
    Ureq(#[from] ureq::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),*/

    // fallback for other boxed errors:
    #[error("internal error: {0}")]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}
