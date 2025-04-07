mod agent;
mod error;

pub use agent::*;
use enclave_vsock::{VsockClientCreateOptions, DEFAULT_ENCLAVE_CID, DEFAULT_ENCLAVE_VSOCK_PORT};
pub use error::*;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    #[builder(default = DEFAULT_ENCLAVE_CID)]
    pub enclave_cid: u32,
    #[builder(default = DEFAULT_ENCLAVE_VSOCK_PORT)]
    pub enclave_vsock_port: u32,
    #[builder(default = vec![])]
    pub pcr0: Vec<u8>,
}

impl From<EnclaveAgentCreateOptions> for VsockClientCreateOptions {
    fn from(options: EnclaveAgentCreateOptions) -> Self {
        VsockClientCreateOptions::builder()
            .enclave_cid(options.enclave_cid)
            .enclave_vsock_port(options.enclave_vsock_port)
            .build()
    }
}

pub fn create_enclave_agent(options: EnclaveAgentCreateOptions) -> Result<Arc<EnclaveAgent>, EnclaveAgentError> {
    let agent = EnclaveAgent::new(options)?;
    Ok(Arc::new(agent))
}
