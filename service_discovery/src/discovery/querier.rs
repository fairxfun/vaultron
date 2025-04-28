use super::VaultronServiceDiscoveryHandler;
use crate::{
    ServiceInstanceAttributesTrait, VaultronServiceDiscoveryError, VaultronServiceInstance,
    VaultronServiceQuerierTrait, VaultronServiceTags,
};

#[async_trait::async_trait]
impl<A: ServiceInstanceAttributesTrait> VaultronServiceQuerierTrait<A> for VaultronServiceDiscoveryHandler<A> {
    async fn get_service_tags(&self) -> Result<VaultronServiceTags, VaultronServiceDiscoveryError> {
        let tags = self.list_resource_tags().await?;
        Ok(tags.try_into()?)
    }

    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance<A>>, VaultronServiceDiscoveryError> {
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
            let batch_instances: Vec<VaultronServiceInstance<A>> = response
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
    ) -> Result<VaultronServiceInstance<A>, VaultronServiceDiscoveryError> {
        let response = self
            .aws_client
            .get_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(instance_id))
            .send()
            .await?;
        match response.instance() {
            Some(instance) => Ok(instance.try_into()?),
            None => Err(VaultronServiceDiscoveryError::InstanceNotFound),
        }
    }
}
