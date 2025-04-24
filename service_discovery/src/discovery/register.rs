use crate::{
    ServiceAttributesTrait, VaultronServiceDiscoveryError, VaultronServiceDiscoveryHandler,
    VaultronServiceRegisterTrait,
};

#[async_trait::async_trait]
impl<A: ServiceAttributesTrait> VaultronServiceRegisterTrait<A> for VaultronServiceDiscoveryHandler<A> {
    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .register_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(self.instance_id.clone()))
            .set_attributes(Some(self.attributes.clone().into_attributes()))
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
