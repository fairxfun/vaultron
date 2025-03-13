mod agent;
mod client;
mod error;

pub use agent::*;
pub use error::*;

use client::EnclaveAgent;
use std::sync::Arc;

pub fn create_enclave_agent(
    options: EnclaveAgentCreateOptions,
) -> Result<Arc<Box<dyn EnclaveAgentTrait>>, EnclaveAgentError> {
    let agent = EnclaveAgent::new(options)?;
    Ok(Arc::new(Box::new(agent) as Box<dyn EnclaveAgentTrait>))
}
