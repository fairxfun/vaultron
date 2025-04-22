use crate::{VaultronServiceDiscoveryError, VaultronServiceDiscoveryHandler, VaultronServiceRegisterTrait};

#[async_trait::async_trait]
impl VaultronServiceRegisterTrait for VaultronServiceDiscoveryHandler {
    async fn register_service(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .register_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(self.instance_id.clone()))
            .set_attributes(Some(self.attributes.attributes.clone()))
            .send()
            .await?;
        Ok(())
    }

    async fn deregister_service(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.aws_client
            .deregister_instance()
            .set_service_id(Some(self.service_id.clone()))
            .set_instance_id(Some(self.instance_id.clone()))
            .send()
            .await?;
        Ok(())
    }
}
