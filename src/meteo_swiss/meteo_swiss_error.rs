use std::fmt;

use crate::grib2::common::grib2_error::Grib2Error;


#[derive(Debug)]
pub enum MeteoSwissError {
    InvalidParameters(String),
    NoForecastRunsFound(),
    ChronoError(chrono::ParseError),
    SerdeError(serde_json::Error),
    Grib2Error(Grib2Error),
    IoError(std::io::Error),
    UreqError(String),
}


impl fmt::Display for MeteoSwissError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeteoSwissError::InvalidParameters(err) => write!(f, "Invalid parameters: {}", err),
            MeteoSwissError::NoForecastRunsFound() => write!(f, "No forecast runs found"),
            MeteoSwissError::ChronoError(err) => write!(f, "Chrono Error: {}", err.to_string()),
            MeteoSwissError::SerdeError(err) => write!(f, "Serde Error: {}", err.to_string()),
            MeteoSwissError::Grib2Error(err) => write!(f, "Grib2 Error: {}", err.to_string()),
            MeteoSwissError::IoError(err) => write!(f, "I/O Error: {}", err.to_string()),
            MeteoSwissError::UreqError(err) => write!(f, "Ureq Error: {}", err),
        }
    }
}


impl From<std::io::Error> for MeteoSwissError {
    fn from(err: std::io::Error) -> Self {
        MeteoSwissError::IoError(err)
    }
}


impl From<Grib2Error> for MeteoSwissError {
    fn from(err: Grib2Error) -> Self {
        MeteoSwissError::Grib2Error(err)
    }
}


impl From<ureq::Error> for MeteoSwissError {
    fn from(err: ureq::Error) -> Self {
        MeteoSwissError::UreqError(err.to_string())
    }
}


impl From<serde_json::Error> for MeteoSwissError {
    fn from(err: serde_json::Error) -> Self {
        MeteoSwissError::SerdeError(err)
    }
}


impl From<chrono::ParseError> for MeteoSwissError {
    fn from(err: chrono::ParseError) -> Self {
        MeteoSwissError::ChronoError(err)
    }
}
