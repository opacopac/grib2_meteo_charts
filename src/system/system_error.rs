use thiserror::Error;


#[derive(Debug, Error)]
pub enum SystemError {
    #[error(transparent)]
    Ureq(#[from] ureq::Error),
}
