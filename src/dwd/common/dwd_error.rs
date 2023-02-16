use crate::grib2::common::grib2_error::Grib2Error;

#[derive(Debug)]
pub enum DwdError {
    Grib2Error(Grib2Error),
    IoError(std::io::Error)
}


impl From<std::io::Error> for DwdError {
    fn from(err: std::io::Error) -> Self {
        return DwdError::IoError(err);
    }
}

impl From<Grib2Error> for DwdError {
    fn from(err: Grib2Error) -> Self {
        return DwdError::Grib2Error(err);
    }
}
