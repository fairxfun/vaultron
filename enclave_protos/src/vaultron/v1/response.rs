pub use crate::gen::vaultron::v1::*;

impl From<status_code::Error> for EnclaveProtoError {
    fn from(value: status_code::Error) -> Self {
        match value {
            status_code::Error::Enclave(error_code) => {
                EnclaveProtoError::try_from(error_code).unwrap_or(EnclaveProtoError::EnclaveErrorUnknownError)
            }
        }
    }
}

impl From<&status_code::Error> for EnclaveProtoError {
    fn from(value: &status_code::Error) -> Self {
        match value {
            status_code::Error::Enclave(error_code) => {
                EnclaveProtoError::try_from(*error_code).unwrap_or(EnclaveProtoError::EnclaveErrorUnknownError)
            }
        }
    }
}

impl StatusCode {
    pub fn success() -> Self {
        Self::builder().success(true).build()
    }

    pub fn error<E>(err: E) -> Self
    where
        E: Into<EnclaveProtoError>,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::Enclave(err.into() as i32))
            .build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::Enclave(
                EnclaveProtoError::EnclaveErrorUnknownError as i32,
            ))
            .error_message(error_message.to_string())
            .build()
    }
}

impl EnclaveResponse {
    pub fn success(attestation_document: Vec<u8>) -> Self {
        Self::builder()
            .code(StatusCode::success())
            .attestation_document(attestation_document)
            .build()
    }

    pub fn error<E>(err: E) -> Self
    where
        E: Into<EnclaveProtoError>,
    {
        Self::builder().code(StatusCode::error(err)).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder().code(StatusCode::unknown_error(error_message)).build()
    }

    pub fn is_success(&self) -> Result<(), EnclaveProtoError> {
        match &self.code {
            Some(code) => {
                if code.success {
                    Ok(())
                } else {
                    match &code.error {
                        Some(error) => Err(error.into()),
                        None => Err(EnclaveProtoError::EnclaveErrorUnknownError),
                    }
                }
            }
            None => Err(EnclaveProtoError::EnclaveErrorUnknownError),
        }
    }
}
