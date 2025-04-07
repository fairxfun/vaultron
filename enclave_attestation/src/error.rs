use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveAttestationError {
    #[error("Invalid certificate chain")]
    InvalidCertificateChain,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid CBOR payload format")]
    InvalidCborPayloadFormat,

    #[error("PCR0 mismatch")]
    Pcr0Mismatch,

    #[error(transparent)]
    X509ParseError(#[from] openssl::error::ErrorStack),

    #[error(transparent)]
    CoseSign1PayloadParseError(#[from] aws_nitro_enclaves_cose::error::CoseError),

    #[error(transparent)]
    SerdeCborError(#[from] serde_cbor::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
