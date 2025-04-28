pub use crate::gen::vaultron::common::v1::*;

impl From<&status_code::Error> for EnclaveError {
    fn from(value: &status_code::Error) -> Self {
        match value {
            status_code::Error::EnclaveError(error_code) => {
                EnclaveError::from_i32(*error_code).unwrap_or(EnclaveError::UnknownError)
            }
            status_code::Error::EnclaveAgentError(_) => EnclaveError::UnknownError,
            status_code::Error::CoordinatorError(_) => EnclaveError::UnknownError,
        }
    }
}

impl From<&status_code::Error> for EnclaveAgentError {
    fn from(value: &status_code::Error) -> Self {
        match value {
            status_code::Error::EnclaveError(_) => EnclaveAgentError::UnknownError,
            status_code::Error::EnclaveAgentError(error_code) => {
                EnclaveAgentError::from_i32(*error_code).unwrap_or(EnclaveAgentError::UnknownError)
            }
            status_code::Error::CoordinatorError(_) => EnclaveAgentError::UnknownError,
        }
    }
}

impl From<&status_code::Error> for CoordinatorError {
    fn from(value: &status_code::Error) -> Self {
        match value {
            status_code::Error::CoordinatorError(error_code) => {
                CoordinatorError::from_i32(*error_code).unwrap_or(CoordinatorError::UnknownError)
            }
            status_code::Error::EnclaveError(_) => CoordinatorError::UnknownError,
            status_code::Error::EnclaveAgentError(_) => CoordinatorError::UnknownError,
        }
    }
}

impl StatusCode {
    pub fn success() -> Self {
        Self::builder().success(true).build()
    }

    pub fn enclave_error<E>(err: E) -> Self
    where
        E: Into<EnclaveError>,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::EnclaveError(err.into() as i32))
            .build()
    }

    pub fn enclave_agent_error<E>(err: E) -> Self
    where
        E: Into<EnclaveAgentError>,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::EnclaveAgentError(err.into() as i32))
            .build()
    }

    pub fn coordinator_error<E>(err: E) -> Self
    where
        E: Into<CoordinatorError>,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::CoordinatorError(err.into() as i32))
            .build()
    }

    pub fn enclave_unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::EnclaveError(EnclaveError::UnknownError as i32))
            .error_message(error_message.to_string())
            .build()
    }

    pub fn enclave_agent_unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::EnclaveAgentError(
                EnclaveAgentError::UnknownError as i32,
            ))
            .error_message(error_message.to_string())
            .build()
    }

    pub fn coordinator_unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .success(false)
            .error(status_code::Error::CoordinatorError(
                CoordinatorError::UnknownError as i32,
            ))
            .error_message(error_message.to_string())
            .build()
    }
}
