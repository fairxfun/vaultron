use crate::error::ServiceDiscoveryError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_servicediscovery::Client;
use aws_types::region::Region;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceRegisterConfig {
    pub region: String,
    pub service_id: String,
    pub instance_id: String,
    pub attributes: HashMap<String, String>,
}

impl VaultronServiceRegisterConfig {
    pub fn validate(&self) -> Result<(), ServiceDiscoveryError> {
        if self.attributes.is_empty() {
            return Err(ServiceDiscoveryError::InvalidAttributes);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceRegister {
    config: VaultronServiceRegisterConfig,
    client: Client,
}

impl VaultronServiceRegister {
    pub async fn new(config: VaultronServiceRegisterConfig) -> Self {
        let region_provider = RegionProviderChain::first_try(Region::new(config.region.clone()));
        let aws_config = aws_config::from_env().region(region_provider).load().await;
        let client = aws_sdk_servicediscovery::Client::new(&aws_config);
        Self { config, client }
    }

    pub async fn register_service(&self) -> Result<(), ServiceDiscoveryError> {
        let attributes = self.config.attributes.clone();
        self.client
            .register_instance()
            .set_service_id(Some(self.config.service_id.clone()))
            .set_instance_id(Some(self.config.instance_id.clone()))
            .set_attributes(Some(attributes))
            .send()
            .await?;
        Ok(())
    }

    pub async fn deregister_service(&self) -> Result<(), ServiceDiscoveryError> {
        self.client
            .deregister_instance()
            .set_service_id(Some(self.config.service_id.clone()))
            .set_instance_id(Some(self.config.instance_id.clone()))
            .send()
            .await?;
        Ok(())
    }
}
