mod discovery;
mod error;
mod types;

pub use discovery::*;
pub use error::*;
pub use types::*;

#[async_trait::async_trait]
pub trait VaultronServiceQuerierTrait<A: ServiceInstanceAttributesTrait>: Send + Sync {
    async fn get_service_tags(&self) -> Result<VaultronServiceTags, VaultronServiceDiscoveryError>;
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance<A>>, VaultronServiceDiscoveryError>;
    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance<A>, VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl<A: ServiceInstanceAttributesTrait> VaultronServiceQuerierTrait<A> for Box<dyn VaultronServiceQuerierTrait<A>> {
    async fn get_service_tags(&self) -> Result<VaultronServiceTags, VaultronServiceDiscoveryError> {
        self.as_ref().get_service_tags().await
    }

    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance<A>>, VaultronServiceDiscoveryError> {
        self.as_ref().list_instances().await
    }

    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance<A>, VaultronServiceDiscoveryError> {
        self.as_ref().get_instance(instance_id).await
    }
}

#[async_trait::async_trait]
pub trait VaultronServiceRegisterTrait<A: ServiceInstanceAttributesTrait>: Send + Sync {
    async fn update_service_tags(&self, tags: VaultronServiceTags) -> Result<(), VaultronServiceDiscoveryError>;
    async fn get_service_tags(&self) -> Result<Option<VaultronServiceTags>, VaultronServiceDiscoveryError>;
    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError>;
    async fn deregister_instance(&self) -> Result<(), VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl<A: ServiceInstanceAttributesTrait> VaultronServiceRegisterTrait<A> for Box<dyn VaultronServiceRegisterTrait<A>> {
    async fn update_service_tags(&self, tags: VaultronServiceTags) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().update_service_tags(tags).await
    }

    async fn get_service_tags(&self) -> Result<Option<VaultronServiceTags>, VaultronServiceDiscoveryError> {
        self.as_ref().get_service_tags().await
    }

    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().register_instance().await
    }

    async fn deregister_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().deregister_instance().await
    }
}
