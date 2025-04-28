use crate::VaultronServiceDiscoveryError;
use aws_sdk_servicediscovery::types::{builders::TagBuilder, Instance, InstanceSummary, Tag};
use std::collections::HashMap;
use std::fmt::Display;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceTags {
    pub pcr0: Vec<u8>,
    pub cluster_public_key: Vec<u8>,
}

impl Display for VaultronServiceTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VaultronServiceTags {{ pcr0: {}, cluster_public_key: {} }}",
            hex::encode(self.pcr0.clone()),
            hex::encode(self.cluster_public_key.clone())
        )
    }
}

impl TryFrom<VaultronServiceTags> for Vec<Tag> {
    type Error = VaultronServiceDiscoveryError;

    fn try_from(tags: VaultronServiceTags) -> Result<Self, Self::Error> {
        let pcr0_tag = TagBuilder::default()
            .set_key(Some("pcr0".to_string()))
            .set_value(Some(hex::encode(tags.pcr0)))
            .build()?;
        let cluster_public_key_tag = TagBuilder::default()
            .set_key(Some("cluster_public_key".to_string()))
            .set_value(Some(hex::encode(tags.cluster_public_key)))
            .build()?;
        Ok(vec![pcr0_tag, cluster_public_key_tag])
    }
}

impl TryFrom<Vec<Tag>> for VaultronServiceTags {
    type Error = VaultronServiceDiscoveryError;

    fn try_from(tags: Vec<Tag>) -> Result<Self, Self::Error> {
        let pcr0 = tags
            .iter()
            .find(|tag| tag.key == "pcr0")
            .ok_or(VaultronServiceDiscoveryError::ServiceTagNotFound)?
            .value();
        let pcr0 = hex::decode(pcr0)?;
        let cluster_public_key = tags
            .iter()
            .find(|tag| tag.key == "cluster_public_key")
            .ok_or(VaultronServiceDiscoveryError::ServiceTagNotFound)?
            .value();
        let cluster_public_key = hex::decode(cluster_public_key)?;
        Ok(Self::builder()
            .pcr0(pcr0)
            .cluster_public_key(cluster_public_key)
            .build())
    }
}

pub trait ServiceInstanceAttributesTrait: Sized + Default + Clone + Send + Sync + 'static {
    fn into_attributes(self) -> HashMap<String, String>;
    fn from_attributes(attributes: HashMap<String, String>) -> Result<Self, VaultronServiceDiscoveryError>;
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceInstance<A: ServiceInstanceAttributesTrait> {
    pub id: String,
    pub attributes: A,
}

impl<A: ServiceInstanceAttributesTrait> TryFrom<&InstanceSummary> for VaultronServiceInstance<A> {
    type Error = VaultronServiceDiscoveryError;

    fn try_from(instance: &InstanceSummary) -> Result<Self, Self::Error> {
        match instance.id.as_ref() {
            Some(id) => Ok(Self {
                id: id.clone(),
                attributes: A::from_attributes(instance.attributes.clone().unwrap_or_default())?,
            }),
            None => Err(VaultronServiceDiscoveryError::InstanceNotFound),
        }
    }
}

impl<A: ServiceInstanceAttributesTrait> TryFrom<&Instance> for VaultronServiceInstance<A> {
    type Error = VaultronServiceDiscoveryError;

    fn try_from(instance: &Instance) -> Result<Self, Self::Error> {
        Ok(Self {
            id: instance.id.clone(),
            attributes: A::from_attributes(instance.attributes.clone().unwrap_or_default())?,
        })
    }
}
