pub use crate::gen::vaultron::enclave::v1::*;

use crate::vaultron::common::v1::CoordinatorError;
use crate::vaultron::common::v1::EnclaveAgentError;
pub use crate::vaultron::common::v1::EnclaveError;
pub use crate::vaultron::common::v1::StatusCode;

impl EnclaveResponse {
    pub fn success(attestation_document: Vec<u8>) -> Self {
        Self::builder()
            .code(StatusCode::success())
            .attestation_document(attestation_document)
            .build()
    }

    pub fn enclave_error<E>(err: E) -> Self
    where
        E: Into<EnclaveError>,
    {
        Self::builder().code(StatusCode::enclave_error(err)).build()
    }

    pub fn enclave_agent_error<E>(err: E) -> Self
    where
        E: Into<EnclaveAgentError>,
    {
        Self::builder().code(StatusCode::enclave_agent_error(err)).build()
    }

    pub fn coordinator_error<E>(err: E) -> Self
    where
        E: Into<CoordinatorError>,
    {
        Self::builder().code(StatusCode::coordinator_error(err)).build()
    }

    pub fn is_success(&self) -> Result<(), EnclaveError> {
        match &self.code {
            Some(code) => {
                if code.success {
                    Ok(())
                } else {
                    match &code.error {
                        Some(error) => Err(error.into()),
                        None => Err(EnclaveError::UnknownError),
                    }
                }
            }
            None => Err(EnclaveError::UnknownError),
        }
    }
}

impl From<EnclaveError> for EnclaveResponse {
    fn from(err: EnclaveError) -> Self {
        Self::enclave_error(err)
    }
}

impl From<EnclaveAgentError> for EnclaveResponse {
    fn from(err: EnclaveAgentError) -> Self {
        Self::enclave_agent_error(err)
    }
}

impl From<CoordinatorError> for EnclaveResponse {
    fn from(err: CoordinatorError) -> Self {
        Self::coordinator_error(err)
    }
}
