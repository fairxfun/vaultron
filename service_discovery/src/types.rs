use crate::ServiceDiscoveryError;
use aws_sdk_servicediscovery::types::{HealthStatus, Instance, InstanceSummary};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VaultronInstance {
    pub id: String,
    pub attributes: HashMap<String, String>,
}

impl TryFrom<&InstanceSummary> for VaultronInstance {
    type Error = ServiceDiscoveryError;

    fn try_from(instance: &InstanceSummary) -> Result<Self, Self::Error> {
        match instance.id.as_ref() {
            Some(id) => Ok(Self {
                id: id.clone(),
                attributes: instance.attributes.clone().unwrap_or_default(),
            }),
            None => Err(ServiceDiscoveryError::InstanceNotFound),
        }
    }
}

impl From<&Instance> for VaultronInstance {
    fn from(instance: &Instance) -> Self {
        Self {
            id: instance.id.clone(),
            attributes: instance.attributes.clone().unwrap_or_default(),
        }
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
