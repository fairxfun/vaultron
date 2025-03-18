pub use crate::gen::vaultron::enclave::v1::*;

impl PingResponse {
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
}
