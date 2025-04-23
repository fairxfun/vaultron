use crate::VaultronServiceDiscoveryError;
use aws_sdk_servicediscovery::types::{HealthStatus, Instance, InstanceSummary};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

pub trait ServiceAttributesTrait: Sized + Default + Clone + Send + Sync + 'static {
    fn into_attributes(self) -> HashMap<String, String>;
    fn from_attributes(attributes: HashMap<String, String>) -> Result<Self, VaultronServiceDiscoveryError>;
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronServiceInstance<A: ServiceAttributesTrait> {
    pub id: String,
    pub attributes: A,
}

impl<A: ServiceAttributesTrait> TryFrom<&InstanceSummary> for VaultronServiceInstance<A> {
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

impl<A: ServiceAttributesTrait> TryFrom<&Instance> for VaultronServiceInstance<A> {
    type Error = VaultronServiceDiscoveryError;

    fn try_from(instance: &Instance) -> Result<Self, Self::Error> {
        Ok(Self {
            id: instance.id.clone(),
            attributes: A::from_attributes(instance.attributes.clone().unwrap_or_default())?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum VaultronInstanceHealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

impl From<&HealthStatus> for VaultronInstanceHealthStatus {
    fn from(health_status: &HealthStatus) -> Self {
        match health_status {
            HealthStatus::Healthy => Self::Healthy,
            HealthStatus::Unhealthy => Self::Unhealthy,
            HealthStatus::UnknownValue => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VaultronInstanceHealthStatusMap {
    pub status: HashMap<String, VaultronInstanceHealthStatus>,
}

impl From<&HashMap<String, HealthStatus>> for VaultronInstanceHealthStatusMap {
    fn from(status: &HashMap<String, HealthStatus>) -> Self {
        Self {
            status: status.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
        }
    }
}
