use crate::imaging::imaging_error::ImagingError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MapTileError {
    #[error(transparent)]
    Imaging(#[from] ImagingError),
}
