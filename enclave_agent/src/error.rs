use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveAgentError {
    #[error("Failed to send request: {0}")]
    VsockClientError(#[from] enclave_vsock::EnclaveVsockClientError),
}
