use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoCommonError {
    #[error("Error finding step nr: {0}")]
    InvalidStepNrError(usize),
}
