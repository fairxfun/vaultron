use enclave_protos::vaultron::common::v1::EnclaveError as EnclaveProtoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveError {
    #[error("Unknown error")]
    UnknownError,

    #[error("Cluster not initialized")]
    ClusterNotInitialized,

    #[error("Cluster already initialized")]
    ClusterAlreadyInitialized,

    #[error("Seed enclave cannot process request")]
    SeedCannotProcessRequest,

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

    #[error("Invalid attestation document error")]
    InvalidAttestationDocumentError,

    #[error("Invalid cluster public key error")]
    InvalidClusterPublicKeyError,

    #[error("NSM API error")]
    NsmApiError,

    #[error(transparent)]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error(transparent)]
    ProtobufDecodeError(#[from] prost::DecodeError),

    #[error(transparent)]
    EnclaveCryptoError(#[from] enclave_crypto::EnclaveCryptoError),

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

    #[error(transparent)]
    AttestationError(#[from] enclave_attestation::EnclaveAttestationError),
}

impl From<EnclaveError> for EnclaveProtoError {
    fn from(value: EnclaveError) -> Self {
        parse_enclave_error(&value)
    }
}

fn parse_enclave_error(err: &EnclaveError) -> EnclaveProtoError {
    match err {
        EnclaveError::UnknownError => EnclaveProtoError::UnknownError,
        EnclaveError::ClusterNotInitialized => EnclaveProtoError::ClusterNotInitializedError,
        EnclaveError::ClusterAlreadyInitialized => EnclaveProtoError::ClusterAlreadyInitializedError,
        EnclaveError::SeedCannotProcessRequest => EnclaveProtoError::SeedCannotProcessRequestError,
        EnclaveError::InvalidRequestError => EnclaveProtoError::InvalidRequestError,
        EnclaveError::InvalidSignatureError => EnclaveProtoError::InvalidSignatureError,
        EnclaveError::InvalidAccountError => EnclaveProtoError::InvalidAccountError,
        EnclaveError::WalletGenerationError => EnclaveProtoError::WalletGenerationError,
        EnclaveError::InvalidParameterError => EnclaveProtoError::InvalidParameterError,
        EnclaveError::InvalidAttestationDocumentError => EnclaveProtoError::InvalidAttestationDocumentError,
        EnclaveError::InvalidClusterPublicKeyError => EnclaveProtoError::InvalidClusterPublicKeyError,
        EnclaveError::NsmApiError => EnclaveProtoError::NsmApiError,
        EnclaveError::ProtobufEncodeError(_) => EnclaveProtoError::ProtobufEncodeError,
        EnclaveError::ProtobufDecodeError(_) => EnclaveProtoError::ProtobufDecodeError,
        EnclaveError::EnclaveCryptoError(_) => EnclaveProtoError::EnclaveCryptoError,
        EnclaveError::PostcardError(_) => EnclaveProtoError::PostcardError,
        EnclaveError::LogError(_) => EnclaveProtoError::LogError,
        EnclaveError::IoError(_) => EnclaveProtoError::IoError,
        EnclaveError::AnyhowError(_) => EnclaveProtoError::AnyhowError,
        EnclaveError::SerdeJsonError(_) => EnclaveProtoError::SerdeJsonError,
        EnclaveError::AttestationError(_) => EnclaveProtoError::AttestationError,
    }
}
