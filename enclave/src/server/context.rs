use crate::{
    common::{EnclaveConfig, EnclaveError},
    data::{EnclaveData, EnclaveKmsData},
};
use anyhow::Result;
use enclave_kmstool::KmsToolTrait;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[cfg(feature = "kmstool_aws_clib_feature")]
use enclave_kmstool::create_kmstool_clib_client;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveServerContext {
    pub config: Arc<EnclaveConfig>,
    pub kms_client: Arc<Box<dyn KmsToolTrait>>,
    pub kms_keys: Arc<EnclaveKmsData>,
    pub enclave_data: Arc<EnclaveData>,
}

impl EnclaveServerContext {
    pub fn new() -> Result<Self, EnclaveError> {
        let config = Arc::new(EnclaveConfig::default());
        let kms_client = create_kms_client()?;
        let kms_keys = Arc::new(EnclaveKmsData::new());
        let enclave_data = Arc::new(EnclaveData::new());
        Ok(EnclaveServerContext::builder()
            .config(config)
            .kms_client(kms_client)
            .kms_keys(kms_keys)
            .enclave_data(enclave_data)
            .build())
    }
}

fn create_kms_client() -> Result<Arc<Box<dyn KmsToolTrait>>, EnclaveError> {
    #[cfg(feature = "kmstool_aws_clib_feature")]
    {
        Ok(create_kmstool_clib_client())
    }

    #[cfg(not(feature = "kmstool_aws_clib_feature"))]
    {
        panic!("KMS client is not available in workflow mode - this code path should not be executed in workflows")
    }
}
