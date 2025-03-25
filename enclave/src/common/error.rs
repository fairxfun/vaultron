use enclave_protos::enclave::v1::{EnclaveProtoError, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveError {
    #[error("Failed to call kms decrypt error: {0}")]
    KmsDecryptError(String),

    #[error("Invalid kms key id error")]
    InvalidKmsKeyIdError,

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

    #[error("Invalid key policies error")]
    InvalidKeyPoliciesError,

    #[error("Invalid key policy error")]
    InvalidKeyPolicyError,

    #[error("Invalid attestation document error")]
    InvalidAttestationDocumentError,

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

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<EnclaveError> for StatusCode {
    fn from(value: EnclaveError) -> Self {
        parse_enclave_error(&value)
    }
}

fn parse_enclave_error(err: &EnclaveError) -> StatusCode {
    match err {
        EnclaveError::KmsDecryptError(_) => EnclaveProtoError::EnclaveErrorKmsDecryptError.into(),
        EnclaveError::InvalidKmsKeyIdError => EnclaveProtoError::EnclaveErrorInvalidKmsKeyIdError.into(),
        EnclaveError::InvalidRequestError => EnclaveProtoError::EnclaveErrorInvalidRequestError.into(),
        EnclaveError::InvalidSignatureError => EnclaveProtoError::EnclaveErrorInvalidSignatureError.into(),
        EnclaveError::InvalidAccountError => EnclaveProtoError::EnclaveErrorInvalidAccountError.into(),
        EnclaveError::WalletGenerationError => EnclaveProtoError::EnclaveErrorWalletGenerationError.into(),
        EnclaveError::InvalidParameterError => EnclaveProtoError::EnclaveErrorInvalidParameterError.into(),
        EnclaveError::InvalidKmsEncryptedDataError => {
            EnclaveProtoError::EnclaveErrorInvalidKmsEncryptedDataError.into()
        }
        EnclaveError::ProtobufEncodeError(_) => EnclaveProtoError::EnclaveErrorProtobufEncodeError.into(),
        EnclaveError::ProtobufDecodeError(_) => EnclaveProtoError::EnclaveErrorProtobufDecodeError.into(),
        EnclaveError::InvalidKeyPoliciesError => EnclaveProtoError::EnclaveErrorInvalidKeyPoliciesError.into(),
        EnclaveError::InvalidKeyPolicyError => EnclaveProtoError::EnclaveErrorInvalidKeyPolicyError.into(),
        EnclaveError::InvalidAttestationDocumentError => {
            EnclaveProtoError::EnclaveErrorInvalidAttestationDocumentError.into()
        }
        EnclaveError::EnclaveKmstoolError(_) => EnclaveProtoError::EnclaveErrorEnclaveKmstoolError.into(),
        EnclaveError::EnclaveWalletError(_) => EnclaveProtoError::EnclaveErrorEnclaveWalletError.into(),
        EnclaveError::PostcardError(_) => EnclaveProtoError::EnclaveErrorPostcardError.into(),
        EnclaveError::LogError(_) => EnclaveProtoError::EnclaveErrorLogError.into(),
        EnclaveError::IoError(_) => EnclaveProtoError::EnclaveErrorIoError.into(),
        EnclaveError::AnyhowError(_) => EnclaveProtoError::EnclaveErrorAnyhowError.into(),
        EnclaveError::SerdeJsonError(_) => EnclaveProtoError::EnclaveErrorSerdeJsonError.into(),
    }
}
