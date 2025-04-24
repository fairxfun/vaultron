use super::CreateOptions;
use crate::{
    EnclaveAgentError, DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT, ENCLAVE_AGENT_GIT_REVISION, ENCLAVE_AGENT_VERSION,
};
use service_discovery::{
    create_service_discovery_register, ServiceAttributesTrait, VaultronServiceDiscoveryError,
    VaultronServiceRegisterTrait,
};
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
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

impl ServiceAttributesTrait for EnclaveAgentServiceAttributes {
    fn into_attributes(self) -> HashMap<String, String> {
        let mut hash_map = HashMap::new();
        hash_map.insert("host".to_string(), self.host.clone());
        hash_map.insert("port".to_string(), self.port.to_string());
        hash_map.insert("version".to_string(), self.version.clone());
        hash_map.insert("git_revision".to_string(), self.git_revision.clone());
        hash_map
    }

    fn from_attributes(attributes: HashMap<String, String>) -> Result<Self, VaultronServiceDiscoveryError> {
        let host = attributes
            .get("host")
            .ok_or(VaultronServiceDiscoveryError::InvalidAttributes)?;
        let port = attributes
            .get("port")
            .ok_or(VaultronServiceDiscoveryError::InvalidAttributes)?
            .parse::<u32>()
            .map_err(|_| VaultronServiceDiscoveryError::InvalidAttributes)?;
        let version = attributes
            .get("version")
            .ok_or(VaultronServiceDiscoveryError::InvalidAttributes)?;
        let git_revision = attributes
            .get("git_revision")
            .ok_or(VaultronServiceDiscoveryError::InvalidAttributes)?;
        Ok(Self::builder()
            .host(host)
            .port(port)
            .version(version)
            .git_revision(git_revision)
            .build())
    }
}

pub(crate) async fn create_enclave_agent_service_register(
    options: &CreateOptions,
) -> Result<Arc<Box<dyn VaultronServiceRegisterTrait<EnclaveAgentServiceAttributes>>>, EnclaveAgentError> {
    let attributes = EnclaveAgentServiceAttributes::builder()
        .host(options.ec2_instance_options.instance_address.clone())
        .port(options.agent_create_options.service_options.port)
        .build();
    let service_register = create_service_discovery_register::<EnclaveAgentServiceAttributes>(
        (&options.agent_create_options.service_options).into(),
        attributes,
        options.ec2_instance_options.instance_id.clone(),
    )
    .await?;
    Ok(service_register)
}
