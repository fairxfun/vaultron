use crate::EnclaveAgentError;
use enclave_protos::enclave::v1::{
    AddKmsKeyRequest, AddKmsKeyResponse, CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, GetEnclavePcrRequest,
    GetEnclavePcrResponse, InitEnclaveRequest, InitEnclaveResponse, PingRequest, PingResponse,
    UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait EnclaveAgentTrait: Send + Sync + Debug {
    async fn reconnect(&self) -> Result<(), EnclaveAgentError>;
    async fn ping(&self, request: PingRequest) -> Result<PingResponse, EnclaveAgentError>;
    async fn init_enclave(&self, request: InitEnclaveRequest) -> Result<InitEnclaveResponse, EnclaveAgentError>;
    async fn add_kms_key(&self, request: AddKmsKeyRequest) -> Result<AddKmsKeyResponse, EnclaveAgentError>;
    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError>;
    async fn get_enclave_pcr(&self, request: GetEnclavePcrRequest) -> Result<GetEnclavePcrResponse, EnclaveAgentError>;
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

    async fn init_enclave(&self, request: InitEnclaveRequest) -> Result<InitEnclaveResponse, EnclaveAgentError> {
        self.as_ref().init_enclave(request).await
    }

    async fn add_kms_key(&self, request: AddKmsKeyRequest) -> Result<AddKmsKeyResponse, EnclaveAgentError> {
        self.as_ref().add_kms_key(request).await
    }

    async fn ping(&self, request: PingRequest) -> Result<PingResponse, EnclaveAgentError> {
        self.as_ref().ping(request).await
    }

    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError> {
        self.as_ref().update_aws_credentials(request).await
    }

    async fn get_enclave_pcr(&self, request: GetEnclavePcrRequest) -> Result<GetEnclavePcrResponse, EnclaveAgentError> {
        self.as_ref().get_enclave_pcr(request).await
    }

    async fn create_enclave_wallet(
        &self,
        request: CreateEnclaveWalletRequest,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveAgentError> {
        self.as_ref().create_enclave_wallet(request).await
    }
}
