use crate::dwd::common::dwd_error::DwdError;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_layer_error::MeteoLayerError;
use crate::metobin::meteobin_error::MeteoBinError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ForecastRendererError {
    #[error("Error reading grid from clct file: {0}")]
    ReadGridFromClctFileError(DwdError),

    #[error("Error reading grid from precip file: {0}")]
    ReadGridFromPrecipFileError(DwdError),

    #[error(transparent)]
    Grib2(#[from] Grib2Error),

    #[error(transparent)]
    Dwd(#[from] DwdError),

    #[error(transparent)]
    MeteoLayerError(#[from] MeteoLayerError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    MeteoBinError(#[from] MeteoBinError),
}
