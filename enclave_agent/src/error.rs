use anyhow::Error as AnyhowError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FairxProxyError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
