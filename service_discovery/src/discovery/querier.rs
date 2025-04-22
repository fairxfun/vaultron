use super::VaultronServiceDiscoveryHandler;
use crate::{
    VaultronInstanceHealthStatusMap, VaultronServiceDiscoveryError, VaultronServiceInstance,
    VaultronServiceQuerierTrait,
};

#[async_trait::async_trait]
impl VaultronServiceQuerierTrait for VaultronServiceDiscoveryHandler {
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance>, VaultronServiceDiscoveryError> {
        let mut instances = Vec::new();
        let mut next_token = None;
        loop {
            let response = self
                .aws_client
                .list_instances()
                .set_service_id(Some(self.service_id.clone()))
                .set_next_token(next_token)
                .send()
                .await?;
            let batch_instances: Vec<VaultronServiceInstance> = response
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

    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance, VaultronServiceDiscoveryError> {
        let response = self
            .aws_client
            .get_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(instance_id))
            .send()
            .await?;
        match response.instance() {
            Some(instance) => Ok(instance.into()),
            None => Err(VaultronServiceDiscoveryError::InstanceNotFound),
        }
    }

    async fn get_instance_health_status(
        &self,
        instance_ids: Vec<String>,
    ) -> Result<VaultronInstanceHealthStatusMap, VaultronServiceDiscoveryError> {
        let response = self
            .aws_client
            .get_instances_health_status()
            .set_service_id(Some(self.service_id.clone()))
            .set_instances(Some(instance_ids))
            .send()
            .await?;
        match response.status() {
            Some(status) => Ok(status.into()),
            None => Err(VaultronServiceDiscoveryError::InstanceStatusUnknown),
        }
    }
}
