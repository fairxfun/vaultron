use anyhow::Error as AnyhowError;
use bip39::Error as Bip39Error;
use fastcrypto::error::FastCryptoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveCryptoError {
    #[error(transparent)]
    EnclaveCryptoAesError(#[from] EnclaveCryptoAesError),

    #[error(transparent)]
    EnclaveCryptoChainKeyError(#[from] EnclaveCryptoChainKeyError),

    #[error(transparent)]
    EnclaveCryptoEciesError(#[from] EnclaveCryptoEciesError),
}

#[derive(Error, Debug)]
pub enum EnclaveCryptoChainKeyError {
    #[error("Mnemonic generation error")]
    MnemonicGenerationError,

    #[error("Mnemonic to seed error")]
    MnemonicToSeedError,

    #[error("Derive key path error")]
    DeriveKeyPathError,

    #[error("Derive key from path error")]
    DeriveKeyFromPathError,

    #[error("Signing key error")]
    SigningKeyError,

    #[error("Wrong key pair error")]
    WrongKeyPairError,

    #[error(transparent)]
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

#[derive(Error, Debug)]
pub enum EnclaveCryptoAesError {
    #[error("Invalid key length error")]
    InvalidKeyLengthError,

    #[error("Invalid AES GCM data length error")]
    InvalidAesGcmDataLengthError,

    #[error("Invalid AES GCM nonce length error")]
    InvalidAesGcmNonceLengthError,

    #[error("AES GCM encryption error")]
    AesGcmEncryptionError,

    #[error("AES GCM decryption error")]
    AesGcmDecryptionError,
}

#[derive(Error, Debug)]
pub enum EnclaveCryptoEciesError {
    #[error("ECIES private key error")]
    EciesPrivateKeyError,

    #[error("ECIES encryption error")]
    EciesEncryptionError,

    #[error("ECIES decryption error")]
    EciesDecryptionError,
}
