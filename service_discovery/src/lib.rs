mod discovery;
mod error;
mod types;

pub use discovery::*;
pub use error::*;
pub use types::*;

#[async_trait::async_trait]
pub trait VaultronServiceQuerierTrait: Send + Sync {
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance>, VaultronServiceDiscoveryError>;
    async fn get_instance(&self, instance_id: String)
        -> Result<VaultronServiceInstance, VaultronServiceDiscoveryError>;
    async fn get_instance_health_status(
        &self,
        instance_ids: Vec<String>,
    ) -> Result<VaultronInstanceHealthStatusMap, VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl VaultronServiceQuerierTrait for Box<dyn VaultronServiceQuerierTrait> {
    async fn list_instances(&self) -> Result<Vec<VaultronServiceInstance>, VaultronServiceDiscoveryError> {
        self.as_ref().list_instances().await
    }

    async fn get_instance(
        &self,
        instance_id: String,
    ) -> Result<VaultronServiceInstance, VaultronServiceDiscoveryError> {
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
pub trait VaultronServiceRegisterTrait: Send + Sync {
    async fn register_service(&self) -> Result<(), VaultronServiceDiscoveryError>;
    async fn deregister_service(&self) -> Result<(), VaultronServiceDiscoveryError>;
}

#[async_trait::async_trait]
impl VaultronServiceRegisterTrait for Box<dyn VaultronServiceRegisterTrait> {
    async fn register_service(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().register_service().await
    }

    async fn deregister_service(&self) -> Result<(), VaultronServiceDiscoveryError> {
        self.as_ref().deregister_service().await
    }
}
