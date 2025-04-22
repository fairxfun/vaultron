use super::EnclaveAgentCreateOptions;
use crate::{
    EnclaveAgentError, DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT, ENCLAVE_AGENT_GIT_REVISION, ENCLAVE_AGENT_VERSION,
};
use service_discovery::{create_service_discovery_register, VaultronServiceAttributes, VaultronServiceRegisterTrait};
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct EnclaveAgentServiceAttributes {
    pub host: String,
    #[builder(default = DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)]
    pub port: u32,
    #[builder(default = ENCLAVE_AGENT_VERSION.to_string())]
    pub version: String,
    #[builder(default = ENCLAVE_AGENT_GIT_REVISION.to_string())]
    pub git_revision: String,
}

impl From<EnclaveAgentServiceAttributes> for VaultronServiceAttributes {
    fn from(attributes: EnclaveAgentServiceAttributes) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert("host".to_string(), attributes.host.clone());
        hash_map.insert("port".to_string(), attributes.port.to_string());
        hash_map.insert("version".to_string(), attributes.version.clone());
        hash_map.insert("git_revision".to_string(), attributes.git_revision.clone());
        VaultronServiceAttributes { attributes: hash_map }
    }
}

pub(crate) async fn create_enclave_agent_service_register(
    options: &EnclaveAgentCreateOptions,
) -> Result<Arc<Box<dyn VaultronServiceRegisterTrait>>, EnclaveAgentError> {
    let instance_id = generate_instance_id().await?;
    let enclave_agent_ip = get_enclave_agent_ip().await?;
    let attributes = EnclaveAgentServiceAttributes::builder()
        .host(enclave_agent_ip)
        .port(options.agent_create_options.service_options.port)
        .build();
    let service_register = create_service_discovery_register(
        (&options.agent_create_options.service_options).into(),
        attributes.into(),
        instance_id,
    )
    .await?;
    Ok(service_register)
}

async fn generate_instance_id() -> Result<String, EnclaveAgentError> {
    // use ec2 instance id as service instance id
    let response = reqwest::get("http://169.254.169.254/latest/meta-data/instance-id").await?;
    let instance_id = response.text().await?;
    Ok(instance_id)
}

async fn get_enclave_agent_ip() -> Result<String, EnclaveAgentError> {
    let response = reqwest::get("http://169.254.169.254/latest/meta-data/local-ipv4").await?;
    let enclave_agent_ip = response.text().await?;
    Ok(enclave_agent_ip)
}
