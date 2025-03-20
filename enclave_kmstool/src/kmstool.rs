use crate::EnclaveKmstoolError;
use enclave_protos::enclave::v1::{InitEnclaveRequest, UpdateAwsCredentialsRequest};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmstoolInitParams {
    pub enable_logging: bool,
    pub proxy_port: u32,
    pub aws_region: String,
}

impl From<InitEnclaveRequest> for KmstoolInitParams {
    fn from(request: InitEnclaveRequest) -> Self {
        KmstoolInitParams::builder()
            .enable_logging(request.enable_logging.unwrap_or(false))
            .proxy_port(request.proxy_port.unwrap_or(8000))
            .aws_region(request.aws_region)
            .build()
    }
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmstoolUpdateAwsCredentialsParams {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
}

impl From<UpdateAwsCredentialsRequest> for KmstoolUpdateAwsCredentialsParams {
    fn from(request: UpdateAwsCredentialsRequest) -> Self {
        KmstoolUpdateAwsCredentialsParams::builder()
            .aws_access_key_id(request.aws_access_key_id)
            .aws_secret_access_key(request.aws_secret_access_key)
            .aws_session_token(request.aws_session_token)
            .build()
    }
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmstoolEncryptParams {
    pub kms_key_id: String,
    pub plaintext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmstoolEncryptResult {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct KmstoolDecryptParams {
    pub kms_key_id: String,
    pub kms_algorithm: String,
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmstoolDecryptResult {
    pub plaintext: Vec<u8>,
}

pub trait KmsToolTrait: Send + Sync + Debug {
    fn init(&self, params: KmstoolInitParams) -> anyhow::Result<(), EnclaveKmstoolError>;
    fn update_aws_credentials(
        &self,
        params: KmstoolUpdateAwsCredentialsParams,
    ) -> anyhow::Result<(), EnclaveKmstoolError>;
    fn encrypt(&self, params: KmstoolEncryptParams) -> anyhow::Result<KmstoolEncryptResult, EnclaveKmstoolError>;
    fn decrypt(&self, params: KmstoolDecryptParams) -> anyhow::Result<KmstoolDecryptResult, EnclaveKmstoolError>;
}

impl KmsToolTrait for Box<dyn KmsToolTrait> {
    fn init(&self, params: KmstoolInitParams) -> anyhow::Result<(), EnclaveKmstoolError> {
        self.as_ref().init(params)
    }

    fn update_aws_credentials(
        &self,
        params: KmstoolUpdateAwsCredentialsParams,
    ) -> anyhow::Result<(), EnclaveKmstoolError> {
        self.as_ref().update_aws_credentials(params)
    }

    fn encrypt(&self, params: KmstoolEncryptParams) -> anyhow::Result<KmstoolEncryptResult, EnclaveKmstoolError> {
        self.as_ref().encrypt(params)
    }

    fn decrypt(&self, params: KmstoolDecryptParams) -> anyhow::Result<KmstoolDecryptResult, EnclaveKmstoolError> {
        self.as_ref().decrypt(params)
    }
}
