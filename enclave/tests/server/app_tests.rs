use std::sync::Arc;
use vaultron_enclave::{EnclaveServer, EnclaveServerContext};

#[tokio::test]
pub async fn test_enclave_start() {
    let context = Arc::new(EnclaveServerContext::new().await.unwrap());
    let _ = EnclaveServer::new(context).unwrap();
}
