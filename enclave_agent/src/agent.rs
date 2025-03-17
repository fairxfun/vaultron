use crate::EnclaveAgentError;
use enclave_protos::enclave::v1::{
    CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, InitKmstoolRequest, InitKmstoolResponse,
    UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    pub cid: u32,
    pub port: u32,
}

#[async_trait::async_trait]
pub trait EnclaveAgentTrait: Send + Sync + Debug {
    async fn kmstool_init(&self, request: InitKmstoolRequest) -> Result<InitKmstoolResponse, EnclaveAgentError>;
    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError>;
    async fn create_enclave_wallet(
        &self,
        request: CreateEnclaveWalletRequest,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveAgentError>;
}

#[async_trait::async_trait]
impl EnclaveAgentTrait for Box<dyn EnclaveAgentTrait> {
    async fn kmstool_init(&self, request: InitKmstoolRequest) -> Result<InitKmstoolResponse, EnclaveAgentError> {
        self.as_ref().kmstool_init(request).await
    }

    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError> {
        self.as_ref().update_aws_credentials(request).await
    }

    async fn create_enclave_wallet(
        &self,
        request: CreateEnclaveWalletRequest,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveAgentError> {
        self.as_ref().create_enclave_wallet(request).await
    }
}
