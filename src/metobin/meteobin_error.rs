use thiserror::Error;


#[derive(Debug, Error)]
pub enum MeteoBinError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
