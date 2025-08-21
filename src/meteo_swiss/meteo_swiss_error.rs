use std::fmt;

use crate::grib2::common::grib2_error::Grib2Error;

#[derive(Debug)]
pub enum MeteoSwissError {
    Grib2Error(Grib2Error),
    IoError(std::io::Error),
    Error(String),
}


impl fmt::Display for MeteoSwissError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeteoSwissError::Grib2Error(err) => write!(f, "Grib2 Error: {}", err.to_string()),
            MeteoSwissError::IoError(err) => write!(f, "I/O Error: {}", err.to_string()),
            MeteoSwissError::Error(err) => write!(f, "Ureq Error: {}", err),
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
        MeteoSwissError::Error(err.to_string())
    }
}
