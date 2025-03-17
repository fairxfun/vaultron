use crate::EnclaveKmstoolError;
use enclave_protos::enclave::v1::{InitKmstoolRequest, UpdateAwsCredentialsRequest};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmsInitRequest {
    pub enable_logging: bool,
    pub proxy_port: u32,
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
    pub kms_key_id: String,
    pub kms_encryption_algorithm: String,
}

impl From<InitKmstoolRequest> for KmsInitRequest {
    fn from(request: InitKmstoolRequest) -> Self {
        KmsInitRequest::builder()
            .enable_logging(request.enable_logging.unwrap_or(false))
            .proxy_port(request.proxy_port.unwrap_or(8000))
            .aws_region(request.aws_region)
            .aws_access_key_id(request.aws_access_key_id)
            .aws_secret_access_key(request.aws_secret_access_key)
            .aws_session_token(request.aws_session_token)
            .kms_key_id(request.kms_key_id)
            .kms_encryption_algorithm(
                request
                    .kms_encryption_algorithm
                    .unwrap_or("SYMMETRIC_DEFAULT".to_string()),
            )
            .build()
    }
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmsUpdateAwsCredentialsRequest {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
}

impl From<UpdateAwsCredentialsRequest> for KmsUpdateAwsCredentialsRequest {
    fn from(request: UpdateAwsCredentialsRequest) -> Self {
        KmsUpdateAwsCredentialsRequest::builder()
            .aws_access_key_id(request.aws_access_key_id)
            .aws_secret_access_key(request.aws_secret_access_key)
            .aws_session_token(request.aws_session_token)
            .build()
    }
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmsEncryptRequest {
    pub plaintext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsEncryptResponse {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmsDecryptRequest {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsDecryptResponse {
    pub plaintext: Vec<u8>,
}

pub trait KmsToolTrait: Send + Sync + Debug {
    fn init(&self, request: KmsInitRequest) -> anyhow::Result<(), EnclaveKmstoolError>;
    fn update_aws_credentials(
        &self,
        request: KmsUpdateAwsCredentialsRequest,
    ) -> anyhow::Result<(), EnclaveKmstoolError>;
    fn encrypt(&self, request: KmsEncryptRequest) -> anyhow::Result<KmsEncryptResponse, EnclaveKmstoolError>;
    fn decrypt(&self, request: KmsDecryptRequest) -> anyhow::Result<KmsDecryptResponse, EnclaveKmstoolError>;
}

impl KmsToolTrait for Box<dyn KmsToolTrait> {
    fn init(&self, request: KmsInitRequest) -> anyhow::Result<(), EnclaveKmstoolError> {
        self.as_ref().init(request)
    }

    fn update_aws_credentials(
        &self,
        request: KmsUpdateAwsCredentialsRequest,
    ) -> anyhow::Result<(), EnclaveKmstoolError> {
        self.as_ref().update_aws_credentials(request)
    }

    fn encrypt(&self, request: KmsEncryptRequest) -> anyhow::Result<KmsEncryptResponse, EnclaveKmstoolError> {
        self.as_ref().encrypt(request)
    }

    fn decrypt(&self, request: KmsDecryptRequest) -> anyhow::Result<KmsDecryptResponse, EnclaveKmstoolError> {
        self.as_ref().decrypt(request)
    }
}
