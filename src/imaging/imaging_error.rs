use thiserror::Error;


#[derive(Debug, Error)]
pub enum ImagingError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error(transparent)]
    Image(#[from] image::ImageError),
}
