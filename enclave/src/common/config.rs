use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveConfig {
    pub log_level: String,
    pub port: u32,
    pub max_buf_len: u32,
}

impl Default for EnclaveConfig {
    fn default() -> EnclaveConfig {
        EnclaveConfig::builder()
            .log_level("info".to_string())
            .port(5000)
            .max_buf_len(8192)
            .build()
    }
}
