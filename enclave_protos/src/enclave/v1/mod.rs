mod config;
mod ping;
mod wallet;

pub use config::*;

use crate::enclave::v1::status_code::Error;

impl StatusCode {
    pub fn success() -> Self {
        Self::builder().success(true).build()
    }
}

impl From<EnclaveError> for StatusCode {
    fn from(value: EnclaveError) -> Self {
        StatusCode::builder()
            .success(false)
            .error(Error::Enclave(value as i32))
            .build()
    }
}
