pub use crate::gen::vaultron::enclave::v1::*;

pub use crate::vaultron::common::v1::EnclaveError;
pub use crate::vaultron::common::v1::StatusCode;

impl EnclaveResponse {
    pub fn success(attestation_document: Vec<u8>) -> Self {
        Self::builder()
            .code(StatusCode::success())
            .attestation_document(attestation_document)
            .build()
    }

    pub fn error<E>(err: E) -> Self
    where
        E: Into<EnclaveError>,
    {
        Self::builder().code(StatusCode::enclave_error(err)).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(StatusCode::enclave_unknown_error(error_message))
            .build()
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
        Self::error(err)
    }
}
