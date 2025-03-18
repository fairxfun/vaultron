mod agent;
mod client;
mod error;

pub use agent::*;
pub use client::*;
use enclave_vsock::{VsockClientCreateOptions, DEFAULT_VSOCK_CID, DEFAULT_VSOCK_PORT};
pub use error::*;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    #[builder(default = DEFAULT_VSOCK_CID)]
    pub server_cid: u32,
    #[builder(default = DEFAULT_VSOCK_PORT)]
    pub server_port: u32,
}

impl From<EnclaveAgentCreateOptions> for VsockClientCreateOptions {
    fn from(options: EnclaveAgentCreateOptions) -> Self {
        VsockClientCreateOptions::builder()
            .server_cid(options.server_cid)
            .server_port(options.server_port)
            .build()
    }
}

pub fn create_enclave_agent(
    options: EnclaveAgentCreateOptions,
) -> Result<Arc<Box<dyn EnclaveAgentTrait>>, EnclaveAgentError> {
    let agent = EnclaveAgent::new(options)?;
    Ok(Arc::new(Box::new(agent) as Box<dyn EnclaveAgentTrait>))
}
