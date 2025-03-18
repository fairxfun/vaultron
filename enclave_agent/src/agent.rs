use crate::EnclaveAgentError;
use enclave_protos::enclave::v1::{
    CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, InitKmstoolRequest, InitKmstoolResponse, PingRequest,
    PingResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait EnclaveAgentTrait: Send + Sync + Debug {
    async fn reconnect(&self) -> Result<(), EnclaveAgentError>;
    async fn ping(&self, request: PingRequest) -> Result<PingResponse, EnclaveAgentError>;
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
    async fn reconnect(&self) -> Result<(), EnclaveAgentError> {
        self.as_ref().reconnect().await
    }

    async fn ping(&self, request: PingRequest) -> Result<PingResponse, EnclaveAgentError> {
        self.as_ref().ping(request).await
    }

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
