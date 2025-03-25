use crate::enclave::v1::status_code::Error;
pub use crate::gen::vaultron::enclave::v1::*;

impl InitEnclaveResponse {
    pub fn success() -> Self {
        Self::builder()
            .code(StatusCode::builder().success(true).build())
            .build()
    }

    pub fn error<E>(code: E) -> Self
    where
        E: Into<StatusCode>,
    {
        Self::builder().code(code.into()).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(
                StatusCode::builder()
                    .success(false)
                    .error(Error::Enclave(EnclaveProtoError::EnclaveErrorUnknownError as i32))
                    .error_message(error_message.to_string())
                    .build(),
            )
            .build()
    }
}

impl AddKmsKeyResponse {
    pub fn success() -> Self {
        Self::builder()
            .code(StatusCode::builder().success(true).build())
            .build()
    }

    pub fn error<E>(code: E) -> Self
    where
        E: Into<StatusCode>,
    {
        Self::builder().code(code.into()).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(
                StatusCode::builder()
                    .success(false)
                    .error(Error::Enclave(EnclaveProtoError::EnclaveErrorUnknownError as i32))
                    .error_message(error_message.to_string())
                    .build(),
            )
            .build()
    }
}

impl UpdateAwsCredentialsResponse {
    pub fn success() -> Self {
        Self::builder()
            .code(StatusCode::builder().success(true).build())
            .build()
    }

    pub fn error<E>(code: E) -> Self
    where
        E: Into<StatusCode>,
    {
        Self::builder().code(code.into()).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(
                StatusCode::builder()
                    .success(false)
                    .error(Error::Enclave(EnclaveProtoError::EnclaveErrorUnknownError as i32))
                    .error_message(error_message.to_string())
                    .build(),
            )
            .build()
    }
}

impl GetEnclavePcrResponse {
    pub fn error<E>(code: E) -> Self
    where
        E: Into<StatusCode>,
    {
        Self::builder().code(code.into()).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(
                StatusCode::builder()
                    .success(false)
                    .error(Error::Enclave(EnclaveProtoError::EnclaveErrorUnknownError as i32))
                    .error_message(error_message.to_string())
                    .build(),
            )
            .build()
    }
}
