mod agent;
mod common;
mod enclave;
mod message;

pub use agent::*;
pub use common::*;

pub const ENCLAVE_AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ENCLAVE_AGENT_GIT_REVISION: &str = env!("VAULTRON_GIT_REVISION");

pub const DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT: u32 = 8000;
pub const DEFAULT_ENCLAVE_CPU_COUNT: u32 = 2;
pub const DEFAULT_ENCLAVE_MEMORY_SIZE: u32 = 1024;
pub const DEFAULT_ENCLAVE_NAME_PREFIX: &str = "vaultron_enclave";
pub const DEFAULT_ENCLAVE_LOG_LEVEL: &str = "info";
pub const DEFAULT_ENCLAVE_ELF_FILE: &str = "/fairx/vaultron_enclave.eif";

pub const DEFAULT_VAULTRON_SERVICE_REGION: &str = "ap-southeast-1";
pub const DEFAULT_VAULTRON_SERVICE_NAMESPACE: &str = "fairx.vaultron";

pub fn enclave_agent_service_name(git_revision: &str) -> String {
    format!("enclave-agent-{}", git_revision)
}
