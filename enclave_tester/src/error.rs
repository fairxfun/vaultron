use prost::DecodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveAgentTesterError {
    #[error("AWS credential request error: {0}")]
    AwsCredentialRequestError(String),

    #[error("Protocol buffer decode error: {0}")]
    ProtobufDecodeError(#[from] DecodeError),

    #[error("Protocol buffer encode error: {0}")]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}
