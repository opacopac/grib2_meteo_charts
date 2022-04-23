#[derive(Debug)]
pub enum Grib2Error {
    InvalidData(String),
    Internal(String)
}


impl std::convert::From<std::io::Error> for Grib2Error {
    fn from(err: std::io::Error) -> Self {
        return Grib2Error::Internal(err.to_string());
    }
}


impl std::convert::From<std::str::Utf8Error> for Grib2Error {
    fn from(err: std::str::Utf8Error) -> Self {
        return Grib2Error::Internal(err.to_string());
    }
}


impl std::convert::From<std::convert::Infallible> for Grib2Error {
    fn from(err: std::convert::Infallible) -> Self {
        return Grib2Error::Internal(err.to_string());
    }
}


impl std::convert::From<image::ImageError> for Grib2Error {
    fn from(err: image::ImageError) -> Self {
        return Grib2Error::Internal(err.to_string());
    }
}
