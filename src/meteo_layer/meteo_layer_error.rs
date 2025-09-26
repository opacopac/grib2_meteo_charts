use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoLayerError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
