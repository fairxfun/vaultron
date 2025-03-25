use enclave_vsock::DEFAULT_VSOCK_PORT;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveConfig {
    pub log_level: String,
    pub port: u32,
}

impl Default for EnclaveConfig {
    fn default() -> Self {
        Self::builder()
            .log_level("info".to_string())
            .port(DEFAULT_VSOCK_PORT)
            .build()
    }
}
