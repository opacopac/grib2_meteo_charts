use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::imaging_error::ImagingError;
use crate::map_tile::map_tile_error::MapTileError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoChartError {
    #[error(transparent)]
    ImagingError(#[from] ImagingError),

    #[error(transparent)]
    MapTileError(#[from] MapTileError),

    #[error(transparent)]
    Grib2Error(#[from] Grib2Error),
}
