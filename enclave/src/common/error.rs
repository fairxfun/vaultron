use enclave_protos::enclave::v1::{EnclaveError as EnclaveProtoError, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveError {
    #[error("Failed to call kms decrypt error: {0}")]
    KmsDecryptError(String),

    #[error("Invalid request format error")]
    InvalidRequestError,

    #[error("Invalid signature error")]
    InvalidSignatureError,

    #[error("Invalid account error")]
    InvalidAccountError,

    #[error("Wallet generation error")]
    WalletGenerationError,

    #[error("Invalid parameter error")]
    InvalidParameterError,

    #[error("Invalid kms encrypted data error")]
    InvalidKmsEncryptedDataError,

    #[error("Protocol buffer encoding error")]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error("Protocol buffer decoding error")]
    ProtobufDecodeError(#[from] prost::DecodeError),

    #[error(transparent)]
    EnclaveKmstoolError(#[from] enclave_kmstool::EnclaveKmstoolError),

    #[error(transparent)]
    EnclaveWalletError(#[from] enclave_wallet::EnclaveWalletError),

    #[error(transparent)]
    PostcardError(#[from] postcard::Error),

    #[error(transparent)]
    LogError(#[from] log::ParseLevelError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl From<EnclaveError> for StatusCode {
    fn from(value: EnclaveError) -> Self {
        parse_enclave_error(&value)
    }
}

fn parse_enclave_error(err: &EnclaveError) -> StatusCode {
    match err {
        EnclaveError::KmsDecryptError(_) => EnclaveProtoError::KmsDecryptError.into(),
        EnclaveError::InvalidRequestError => EnclaveProtoError::InvalidRequestError.into(),
        EnclaveError::InvalidSignatureError => EnclaveProtoError::InvalidSignatureError.into(),
        EnclaveError::InvalidAccountError => EnclaveProtoError::InvalidAccountError.into(),
        EnclaveError::WalletGenerationError => EnclaveProtoError::WalletGenerationError.into(),
        EnclaveError::InvalidParameterError => EnclaveProtoError::InvalidParameterError.into(),
        EnclaveError::InvalidKmsEncryptedDataError => EnclaveProtoError::InvalidKmsEncryptedDataError.into(),
        EnclaveError::ProtobufEncodeError(_) => EnclaveProtoError::ProtobufEncodeError.into(),
        EnclaveError::ProtobufDecodeError(_) => EnclaveProtoError::ProtobufDecodeError.into(),
        EnclaveError::EnclaveKmstoolError(_) => EnclaveProtoError::EnclaveKmstoolError.into(),
        EnclaveError::EnclaveWalletError(_) => EnclaveProtoError::EnclaveWalletError.into(),
        EnclaveError::PostcardError(_) => EnclaveProtoError::PostcardError.into(),
        EnclaveError::LogError(_) => EnclaveProtoError::LogError.into(),
        EnclaveError::IoError(_) => EnclaveProtoError::IoError.into(),
        EnclaveError::AnyhowError(_) => EnclaveProtoError::AnyhowError.into(),
    }
}
