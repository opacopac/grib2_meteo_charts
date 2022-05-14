#[derive(Debug)]
pub enum NetCdfError {
    InvalidData(String),
    Internal(String)
}


impl std::convert::From<std::io::Error> for NetCdfError {
    fn from(err: std::io::Error) -> Self {
        return NetCdfError::Internal(err.to_string());
    }
}


impl std::convert::From<std::str::Utf8Error> for NetCdfError {
    fn from(err: std::str::Utf8Error) -> Self {
        return NetCdfError::Internal(err.to_string());
    }
}


impl std::convert::From<std::convert::Infallible> for NetCdfError {
    fn from(err: std::convert::Infallible) -> Self {
        return NetCdfError::Internal(err.to_string());
    }
}


impl std::convert::From<image::ImageError> for NetCdfError {
    fn from(err: image::ImageError) -> Self {
        return NetCdfError::Internal(err.to_string());
    }
}
