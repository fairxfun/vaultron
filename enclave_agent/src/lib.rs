mod agent;
mod common;
mod enclave;
mod message;

pub use agent::*;
pub use common::*;

pub const GIT_REVISION: &str = env!("VAULTRON_GIT_REVISION");

pub const DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT: u32 = 8000;
pub const DEFAULT_ENCLAVE_CPU_COUNT: u32 = 2;
pub const DEFAULT_ENCLAVE_MEMORY_SIZE: u32 = 1024;
pub const DEFAULT_ENCLAVE_NAME_PREFIX: &str = "vaultron_enclave";
pub const DEFAULT_ENCLAVE_LOG_LEVEL: &str = "info";
pub const DEFAULT_ENCLAVE_ELF_FILE: &str = "/fairx/vaultron_enclave.eif";
