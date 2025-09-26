use crate::grib2::common::grib2_error::Grib2Error;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum DwdError {
    #[error(transparent)]
    Grib2Error(#[from] Grib2Error),
}
