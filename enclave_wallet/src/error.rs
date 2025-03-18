use anyhow::Error as AnyhowError;
use bip39::Error as Bip39Error;
use fastcrypto::error::FastCryptoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveWalletError {
    #[error("Wallet generation error")]
    WalletGenerationError,

    #[error("")]
    FastCryptoError(#[from] FastCryptoError),

    #[error(transparent)]
    Bip39Error(#[from] Bip39Error),

    #[error(transparent)]
    SuiError(#[from] sui_types::error::SuiError),

    #[error(transparent)]
    Bip32Error(#[from] bip32::Error),

    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
