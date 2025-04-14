use typed_builder::TypedBuilder;

#[derive(Debug, Clone)]
pub enum EnclaveType {
    EnclaveSeed,
    EnclaveWorker,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveAttributes {
    pub tag: String,
    pub enclave_pcr0: String,
    pub enclave_type: EnclaveType,
}
