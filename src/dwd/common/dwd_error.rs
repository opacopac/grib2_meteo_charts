use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_layer_error::MeteoLayerError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum DwdError {
    #[error(transparent)]
    Grib2Error(#[from] Grib2Error),

    #[error(transparent)]
    MeteoLayerError(#[from] MeteoLayerError),
}
