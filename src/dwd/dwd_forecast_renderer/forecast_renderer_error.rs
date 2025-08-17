use std::fmt;

use crate::dwd::common::dwd_error::DwdError;
use crate::grib2::common::grib2_error::Grib2Error;

#[derive(Debug)]
pub enum ForecastRendererError {
    Grib2(Grib2Error),
    Dwd(DwdError),
    IoError(std::io::Error),
    ReadGridFromClctFileError(DwdError),
    ReadGridFromPrecipFileError(DwdError),
}


impl fmt::Display for ForecastRendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ForecastRendererError::Grib2(err) => write!(f, "Forecast Renderer Error: {}", err.to_string()),
            ForecastRendererError::Dwd(err) => write!(f, "Forecast Renderer Error: {}", err.to_string()),
            ForecastRendererError::IoError(err) => write!(f, "Forecast Renderer Error: {}", err.to_string()),
            ForecastRendererError::ReadGridFromClctFileError(err)
                => write!(f, "Error reading grid from clct file: {}", err.to_string()),
            ForecastRendererError::ReadGridFromPrecipFileError(err)
            => write!(f, "Error reading grid from precip file: {}", err.to_string()),
        }
    }
}


impl From<Grib2Error> for ForecastRendererError {
    fn from(err: Grib2Error) -> Self {
        return ForecastRendererError::Grib2(err);
    }
}

impl From<DwdError> for ForecastRendererError {
    fn from(err: DwdError) -> Self {
        return ForecastRendererError::Dwd(err);
    }
}

impl From<std::io::Error> for ForecastRendererError {
    fn from(err: std::io::Error) -> Self {
        return ForecastRendererError::IoError(err);
    }
}
