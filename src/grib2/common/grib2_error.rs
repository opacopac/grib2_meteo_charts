use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum Grib2Error {
    InvalidData(String),
    Internal(String)
}


impl fmt::Display for Grib2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grib2Error::InvalidData(msg) => write!(f, "Invalid Grib2 Data: {}", msg),
            Grib2Error::Internal(msg) => write!(f, "Internal Grib2 Error: {}", msg)
        }
    }
}

// from std::error::Error

impl From<ureq::Error> for Grib2Error {
    fn from(err: ureq::Error) -> Self {
        Grib2Error::Internal(err.to_string())
    }
}


impl From<std::io::Error> for Grib2Error {
    fn from(err: std::io::Error) -> Self {
        Grib2Error::Internal(err.to_string())
    }
}


impl From<std::str::Utf8Error> for Grib2Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Grib2Error::Internal(err.to_string())
    }
}


impl From<std::convert::Infallible> for Grib2Error {
    fn from(err: std::convert::Infallible) -> Self {
        Grib2Error::Internal(err.to_string())
    }
}


impl From<image::ImageError> for Grib2Error {
    fn from(err: image::ImageError) -> Self {
        Grib2Error::Internal(err.to_string())
    }
}
