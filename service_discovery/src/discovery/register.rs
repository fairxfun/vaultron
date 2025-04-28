use crate::{
    ServiceInstanceAttributesTrait, VaultronServiceDiscoveryError, VaultronServiceDiscoveryHandler,
    VaultronServiceRegisterTrait, VaultronServiceTags,
};

#[async_trait::async_trait]
impl<A: ServiceInstanceAttributesTrait> VaultronServiceRegisterTrait<A> for VaultronServiceDiscoveryHandler<A> {
    async fn update_service_tags(&self, tags: VaultronServiceTags) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .tag_resource()
            .set_resource_arn(Some(self.service_arn.clone()))
            .set_tags(Some(tags.try_into()?))
            .send()
            .await?;
        Ok(())
    }

    async fn get_service_tags(&self) -> Result<Option<VaultronServiceTags>, VaultronServiceDiscoveryError> {
        let tags = self.list_resource_tags().await?;
        if tags.is_empty() {
            Ok(None)
        } else {
            Ok(Some(tags.try_into()?))
        }
    }

    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .register_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(self.instance_id.clone()))
            .set_attributes(Some(self.instance_attributes.clone().into_attributes()))
            .send()
            .await?;
        Ok(())
    }

    async fn deregister_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .deregister_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(self.instance_id.clone()))
            .send()
            .await?;
        Ok(())
    }
}
