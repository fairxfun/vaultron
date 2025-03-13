use anyhow::Error as AnyhowError;
use bip39::Error as Bip39Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveWalletError {
    #[error("Invalid account error")]
    InvalidAccountError,

    #[error("Wallet generation error")]
    WalletGenerationError,

    #[error(transparent)]
    Bip39Error(#[from] Bip39Error),

    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
