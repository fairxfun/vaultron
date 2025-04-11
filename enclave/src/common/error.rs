use enclave_protos::vaultron::v1::EnclaveProtoError;
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
        EnclaveError::UnknownError => EnclaveProtoError::EnclaveErrorUnknownError,
        EnclaveError::ClusterNotInitialized => EnclaveProtoError::EnclaveErrorClusterNotInitializedError,
        EnclaveError::ClusterAlreadyInitialized => EnclaveProtoError::EnclaveErrorClusterAlreadyInitializedError,
        EnclaveError::SeedCannotProcessRequest => EnclaveProtoError::EnclaveErrorSeedCannotProcessRequestError,
        EnclaveError::InvalidRequestError => EnclaveProtoError::EnclaveErrorInvalidRequestError,
        EnclaveError::InvalidSignatureError => EnclaveProtoError::EnclaveErrorInvalidSignatureError,
        EnclaveError::InvalidAccountError => EnclaveProtoError::EnclaveErrorInvalidAccountError,
        EnclaveError::WalletGenerationError => EnclaveProtoError::EnclaveErrorWalletGenerationError,
        EnclaveError::InvalidParameterError => EnclaveProtoError::EnclaveErrorInvalidParameterError,
        EnclaveError::InvalidAttestationDocumentError => EnclaveProtoError::EnclaveErrorInvalidAttestationDocumentError,
        EnclaveError::InvalidClusterPublicKeyError => EnclaveProtoError::EnclaveErrorInvalidClusterPublicKeyError,
        EnclaveError::NsmApiError => EnclaveProtoError::EnclaveErrorNsmApiError,
        EnclaveError::ProtobufEncodeError(_) => EnclaveProtoError::EnclaveErrorProtobufEncodeError,
        EnclaveError::ProtobufDecodeError(_) => EnclaveProtoError::EnclaveErrorProtobufDecodeError,
        EnclaveError::EnclaveCryptoError(_) => EnclaveProtoError::EnclaveErrorEnclaveCryptoError,
        EnclaveError::PostcardError(_) => EnclaveProtoError::EnclaveErrorPostcardError,
        EnclaveError::LogError(_) => EnclaveProtoError::EnclaveErrorLogError,
        EnclaveError::IoError(_) => EnclaveProtoError::EnclaveErrorIoError,
        EnclaveError::AnyhowError(_) => EnclaveProtoError::EnclaveErrorAnyhowError,
        EnclaveError::SerdeJsonError(_) => EnclaveProtoError::EnclaveErrorSerdeJsonError,
        EnclaveError::AttestationError(_) => EnclaveProtoError::EnclaveErrorAttestationError,
    }
}
