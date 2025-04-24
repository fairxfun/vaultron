mod discovery;
mod error;
mod types;

pub use discovery::*;
pub use error::*;
pub use types::*;

#[async_trait::async_trait]
pub trait VaultronServiceQuerierTrait<A: ServiceAttributesTrait>: Send + Sync {
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance<A>>, VaultronServiceDiscoveryError>;
    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance<A>, VaultronServiceDiscoveryError>;
    async fn get_instance_health_status(
        &self,
        instance_ids: Vec<String>,
    ) -> Result<VaultronInstanceHealthStatusMap, VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl<A: ServiceAttributesTrait> VaultronServiceQuerierTrait<A> for Box<dyn VaultronServiceQuerierTrait<A>> {
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance<A>>, VaultronServiceDiscoveryError> {
        self.as_ref().list_instances().await
    }

    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance<A>, VaultronServiceDiscoveryError> {
        self.as_ref().get_instance(instance_id).await
    }

    async fn get_instance_health_status(
        &self,
        instance_ids: Vec<String>,
    ) -> Result<VaultronInstanceHealthStatusMap, VaultronServiceDiscoveryError> {
        self.as_ref().get_instance_health_status(instance_ids).await
    }
}

#[async_trait::async_trait]
pub trait VaultronServiceRegisterTrait<A: ServiceAttributesTrait>: Send + Sync {
    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError>;
    async fn deregister_instance(&self) -> Result<(), VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl<A: ServiceAttributesTrait> VaultronServiceRegisterTrait<A> for Box<dyn VaultronServiceRegisterTrait<A>> {
    async fn register_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().register_instance().await
    }

    async fn deregister_instance(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().deregister_instance().await
    }
}
