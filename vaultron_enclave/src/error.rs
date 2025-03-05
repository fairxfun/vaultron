use anyhow::Error as AnyhowError;
use log::ParseLevelError as LogError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FairxEnclaveError {
    #[error("Failed to call kms decrypt: {0}")]
    KmsDecryptError(String),
    #[error(transparent)]
    LogError(#[from] LogError),
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
