use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_layer::meteo_layer_error::MeteoLayerError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoSwissError {
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("No forecast runs found")]
    NoForecastRunsFound(),

    #[error(transparent)]
    ChronoError(#[from] chrono::ParseError),

    #[error(transparent)]
    Grib2Error(#[from] Grib2Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UreqError(#[from] ureq::Error),

    #[error(transparent)]
    MeteoLayerError(#[from] MeteoLayerError),
}
