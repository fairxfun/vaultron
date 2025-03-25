use enclave_protos::enclave::v1::AddKmsKeyRequest;
use std::collections::HashMap;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveKmsKey {
    pub kms_key_id: String,
    pub kms_algorithm: String,
}

impl From<AddKmsKeyRequest> for EnclaveKmsKey {
    fn from(request: AddKmsKeyRequest) -> Self {
        Self::builder()
            .kms_key_id(request.kms_key_id)
            .kms_algorithm(request.kms_algorithm.unwrap_or("SYMMETRIC_DEFAULT".to_string()))
            .build()
    }
}

#[derive(Debug, Default)]
pub struct EnclaveKmsData {
    aws_region: RwLock<String>,
    key_ids: RwLock<HashMap<String, EnclaveKmsKey>>,
}

impl EnclaveKmsData {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_aws_region(&self, aws_region: String) {
        let mut region = self.aws_region.write().await;
        *region = aws_region
    }

    pub async fn get_aws_region(&self) -> String {
        let aws_region = self.aws_region.read().await;
        aws_region.clone()
    }

    pub async fn add_key(&self, key: EnclaveKmsKey) {
        let mut key_ids = self.key_ids.write().await;
        key_ids.insert(key.kms_key_id.clone(), key);
    }

    pub async fn remove_key(&self, key_id: String) {
        let mut key_ids = self.key_ids.write().await;
        key_ids.remove(&key_id);
    }

    pub async fn has_key(&self, key_id: &str) -> bool {
        let key_ids = self.key_ids.read().await;
        key_ids.contains_key(key_id)
    }

    pub async fn get_key(&self, key_id: &str) -> Option<EnclaveKmsKey> {
        let key_ids = self.key_ids.read().await;
        key_ids.get(key_id).cloned()
    }
}
