use super::EnclaveAgentError;

pub const ENCLAVE_AGENT_VERSION: &str = env!("VAULTRON_VERSION");
pub const ENCLAVE_AGENT_GIT_REVISION: &str = env!("VAULTRON_GIT_REVISION");

pub const DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT: u32 = 8000;
pub const DEFAULT_ENCLAVE_CPU_COUNT: u32 = 2;
pub const DEFAULT_ENCLAVE_MEMORY_SIZE: u32 = 1024;
pub const DEFAULT_ENCLAVE_NAME_PREFIX: &str = "vaultron-enclave";
pub const DEFAULT_ENCLAVE_LOG_LEVEL: &str = "info";
pub const DEFAULT_ENCLAVE_ELF_FILE: &str = "/fairx/vaultron_enclave.eif";

pub const DEFAULT_VAULTRON_SERVICE_REGION: &str = "ap-southeast-1";
pub const DEFAULT_VAULTRON_SERVICE_NAMESPACE: &str = "fairx.vaultron";

pub fn enclave_agent_service_name(version: &str) -> String {
    format!("enclave-agent-{}", version)
}

pub fn enclave_name(instance_id: &str, cid: u32) -> String {
    format!("{}-{}-{}", instance_id, DEFAULT_ENCLAVE_NAME_PREFIX, cid)
}

pub async fn get_ec2_instance_id() -> Result<String, EnclaveAgentError> {
    let response = reqwest::get("http://169.254.169.254/latest/meta-data/instance-id").await?;
    let instance_id = response.text().await?;
    Ok(instance_id)
}

pub async fn get_ec2_instance_ip() -> Result<String, EnclaveAgentError> {
    let response = reqwest::get("http://169.254.169.254/latest/meta-data/local-ipv4").await?;
    let enclave_instance_ip = response.text().await?;
    Ok(enclave_instance_ip)
}
