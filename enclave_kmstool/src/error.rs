use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveKmstoolError {
    #[error("Failed to call kms decrypt: {0}")]
    KmsDecryptError(String),

    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
