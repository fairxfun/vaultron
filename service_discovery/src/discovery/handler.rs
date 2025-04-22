use crate::{VaultronServiceDiscoveryError, VaultronServiceQuerierTrait, VaultronServiceRegisterTrait};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_servicediscovery::{
    types::{NamespaceFilter, NamespaceFilterName, ServiceFilter, ServiceFilterName},
    Client,
};
use aws_types::region::Region;
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceConfig {
    pub region: String,
    pub namespace: String,
    pub service_name: String,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct VaultronServiceAttributes {
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceDiscoveryHandler {
    pub(crate) attributes: VaultronServiceAttributes,
    pub(crate) service_id: String,
    pub(crate) instance_id: String,
    pub(crate) aws_client: Client,
}

pub async fn create_service_discovery_querier(
    config: VaultronServiceConfig,
) -> Result<Arc<Box<dyn VaultronServiceQuerierTrait>>, VaultronServiceDiscoveryError> {
    let handler =
        VaultronServiceDiscoveryHandler::new(config, VaultronServiceAttributes::default(), "".to_string()).await?;
    Ok(Arc::new(Box::new(handler) as Box<dyn VaultronServiceQuerierTrait>))
}

pub async fn create_service_discovery_register(
    config: VaultronServiceConfig,
    attributes: VaultronServiceAttributes,
    instance_id: String,
) -> Result<Arc<Box<dyn VaultronServiceRegisterTrait>>, VaultronServiceDiscoveryError> {
    let handler = VaultronServiceDiscoveryHandler::new(config, attributes, instance_id).await?;
    Ok(Arc::new(Box::new(handler) as Box<dyn VaultronServiceRegisterTrait>))
}

impl VaultronServiceDiscoveryHandler {
    async fn new(
        config: VaultronServiceConfig,
        attributes: VaultronServiceAttributes,
        instance_id: String,
    ) -> Result<Self, VaultronServiceDiscoveryError> {
        let region_provider = RegionProviderChain::first_try(Region::new(config.region.clone()));
        let aws_config = aws_config::from_env().region(region_provider).load().await;
        let aws_client = aws_sdk_servicediscovery::Client::new(&aws_config);
        let namespace_id = get_namespace_id(&aws_client, &config.namespace).await?;
        let service_id = get_service_id(&aws_client, &namespace_id, &config.service_name).await?;
        Ok(Self {
            attributes,
            service_id,
            instance_id,
            aws_client,
        })
    }
}

async fn get_namespace_id(aws_client: &Client, namespace_name: &str) -> Result<String, VaultronServiceDiscoveryError> {
    let namespace_filter = NamespaceFilter::builder()
        .set_name(Some(NamespaceFilterName::Name))
        .set_values(Some(vec![namespace_name.to_string()]))
        .build()?;
    let response = aws_client
        .list_namespaces()
        .set_filters(Some(vec![namespace_filter]))
        .send()
        .await?;
    let namespace_id = response
        .namespaces()
        .first()
        .ok_or(VaultronServiceDiscoveryError::NamespaceNotFound)?
        .id()
        .ok_or(VaultronServiceDiscoveryError::NamespaceNotFound)?;
    Ok(namespace_id.to_string())
}

async fn get_service_id(
    aws_client: &Client,
    namespace_id: &str,
    service_name: &str,
) -> Result<String, VaultronServiceDiscoveryError> {
    let service_filter = ServiceFilter::builder()
        .set_name(Some(ServiceFilterName::NamespaceId))
        .set_values(Some(vec![namespace_id.to_string()]))
        .build()?;
    let response = aws_client
        .list_services()
        .set_filters(Some(vec![service_filter]))
        .send()
        .await?;
    let service_id = response
        .services()
        .iter()
        .find(|service| service.name() == Some(service_name))
        .ok_or(VaultronServiceDiscoveryError::ServiceNotFound)?
        .id()
        .ok_or(VaultronServiceDiscoveryError::ServiceNotFound)?;
    Ok(service_id.to_string())
}
