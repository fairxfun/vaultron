use crate::enclave::v1::status_code::Error;
pub use crate::gen::vaultron::enclave::v1::*;

impl CreateEnclaveWalletResponse {
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
