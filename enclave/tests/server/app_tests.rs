use crate::enclave_mock::MockKmstoolClient;
use enclave_kmstool::KmsToolTrait;
use std::sync::Arc;
use vaultron_enclave::{
    common::EnclaveConfig,
    data::{EnclaveData, EnclaveKmsData},
    server::{EnclaveServer, EnclaveServerContext},
};

#[tokio::test]
pub async fn test_enclave_start() {
    let kmstool_client = MockKmstoolClient::new();
    let kmstool_client = Arc::new(Box::new(kmstool_client) as Box<dyn KmsToolTrait>);
    let config = Arc::new(EnclaveConfig::default());
    let context = Arc::new(
        EnclaveServerContext::builder()
            .config(config)
            .kms_client(kmstool_client)
            .kms_keys(Arc::new(EnclaveKmsData::new()))
            .enclave_data(Arc::new(EnclaveData::new()))
            .build(),
    );
    let _ = EnclaveServer::new(context).unwrap();
}
