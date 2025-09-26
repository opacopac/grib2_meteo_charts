use crate::imaging::imaging_error::ImagingError;
use crate::meteo_chart::map_tile::map_tile_error::MapTileError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoChartError {
    #[error(transparent)]
    ImagingError(#[from] ImagingError),

    #[error(transparent)]
    MapTileError(#[from] MapTileError),
}
