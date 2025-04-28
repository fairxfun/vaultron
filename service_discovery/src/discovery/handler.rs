use crate::{
    ServiceInstanceAttributesTrait, VaultronServiceDiscoveryError, VaultronServiceQuerierTrait,
    VaultronServiceRegisterTrait,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_servicediscovery::{
    types::{NamespaceFilter, NamespaceFilterName, ServiceFilter, ServiceFilterName, Tag},
    Client,
};
use aws_types::region::Region;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceConfig {
    pub region: String,
    pub namespace: String,
    pub service_name: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceDiscoveryHandler<A: ServiceInstanceAttributesTrait> {
    pub(crate) instance_attributes: A,
    pub(crate) service_id: String,
    pub(crate) service_arn: String,
    pub(crate) instance_id: String,
    pub(crate) aws_client: Client,
}

pub async fn create_service_discovery_querier<A: ServiceInstanceAttributesTrait>(
    config: VaultronServiceConfig,
) -> Result<Arc<Box<dyn VaultronServiceQuerierTrait<A>>>, VaultronServiceDiscoveryError> {
    let handler = VaultronServiceDiscoveryHandler::new(config, A::default(), "".to_string()).await?;
    Ok(Arc::new(Box::new(handler) as Box<dyn VaultronServiceQuerierTrait<A>>))
}

pub async fn create_service_discovery_register<A: ServiceInstanceAttributesTrait>(
    config: VaultronServiceConfig,
    attributes: A,
    instance_id: String,
) -> Result<Arc<Box<dyn VaultronServiceRegisterTrait<A>>>, VaultronServiceDiscoveryError> {
    let handler = VaultronServiceDiscoveryHandler::new(config, attributes, instance_id).await?;
    Ok(Arc::new(Box::new(handler) as Box<dyn VaultronServiceRegisterTrait<A>>))
}

impl<A: ServiceInstanceAttributesTrait> VaultronServiceDiscoveryHandler<A> {
    async fn new(
        config: VaultronServiceConfig,
        instance_attributes: A,
        instance_id: String,
    ) -> Result<Self, VaultronServiceDiscoveryError> {
        let region_provider = RegionProviderChain::first_try(Region::new(config.region.clone()));
        let aws_config = aws_config::from_env().region(region_provider).load().await;
        let aws_client = aws_sdk_servicediscovery::Client::new(&aws_config);
        let namespace_id = get_namespace_id(&aws_client, &config.namespace).await?;
        let (service_id, service_arn) =
            get_service_id_and_arn(&aws_client, &namespace_id, &config.service_name).await?;
        Ok(Self {
            instance_attributes,
            service_id,
            service_arn,
            instance_id,
            aws_client,
        })
    }

    pub(crate) async fn list_resource_tags(&self) -> Result<Vec<Tag>, VaultronServiceDiscoveryError> {
        let response = self
            .aws_client
            .list_tags_for_resource()
            .set_resource_arn(Some(self.service_arn.clone()))
            .send()
            .await?;
        Ok(response.tags().to_vec())
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

async fn get_service_id_and_arn(
    aws_client: &Client,
    namespace_id: &str,
    service_name: &str,
) -> Result<(String, String), VaultronServiceDiscoveryError> {
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
        .ok_or(VaultronServiceDiscoveryError::ServiceNotFound)?;
    match (service_id.id(), service_id.arn()) {
        (Some(id), Some(arn)) => Ok((id.to_string(), arn.to_string())),
        _ => Err(VaultronServiceDiscoveryError::ServiceIdOrArnNotFound),
    }
}
