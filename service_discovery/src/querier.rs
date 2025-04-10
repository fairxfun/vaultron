use crate::{ServiceDiscoveryError, VaultronInstance, VaultronInstanceHealthStatusMap};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_servicediscovery::Client;
use aws_types::region::Region;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceQuerierConfig {
    pub region: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceQuerier {
    client: Client,
}

impl VaultronServiceQuerier {
    pub async fn new(config: VaultronServiceQuerierConfig) -> Self {
        let region_provider = RegionProviderChain::first_try(Region::new(config.region.clone()));
        let aws_config = aws_config::from_env().region(region_provider).load().await;
        let client = aws_sdk_servicediscovery::Client::new(&aws_config);
        Self { client }
    }

    pub async fn list_instances(&self, service_id: String) -> Result<Vec<VaultronInstance>, ServiceDiscoveryError> {
        let mut instances = Vec::new();
        let mut next_token = None;
        loop {
            let response = self
                .client
                .list_instances()
                .set_service_id(Some(service_id.clone()))
                .set_next_token(next_token)
                .send()
                .await?;
            let batch_instances: Vec<VaultronInstance> = response
                .instances()
                .iter()
                .flat_map(|instance| instance.try_into())
                .collect();
            instances.extend(batch_instances);
            match response.next_token() {
                Some(token) => next_token = Some(token.to_string()),
                None => break,
            }
        }
        Ok(instances)
    }

    pub async fn get_instance(
        &self,
        service_id: String,
        instance_id: String,
    ) -> Result<VaultronInstance, ServiceDiscoveryError> {
        let response = self
            .client
            .get_instance()
            .set_service_id(Some(service_id))
            .set_instance_id(Some(instance_id))
            .send()
            .await?;
        match response.instance() {
            Some(instance) => Ok(instance.into()),
            None => Err(ServiceDiscoveryError::InstanceNotFound),
        }
    }

    pub async fn get_instance_health_status(
        &self,
        service_id: String,
        instance_ids: Vec<String>,
    ) -> Result<VaultronInstanceHealthStatusMap, ServiceDiscoveryError> {
        let response = self
            .client
            .get_instances_health_status()
            .set_service_id(Some(service_id))
            .set_instances(Some(instance_ids))
            .send()
            .await?;
        match response.status() {
            Some(status) => Ok(status.into()),
            None => Err(ServiceDiscoveryError::InstanceStatusUnknown),
        }
    }
}
