use super::MessageHandler;
use crate::common::EnclaveError;
use enclave_protos::enclave::v1::{
    AddKmsKeyRequest, AddKmsKeyResponse, InitEnclaveRequest, InitEnclaveResponse, UpdateAwsCredentialsRequest,
    UpdateAwsCredentialsResponse,
};
use log::info;

impl MessageHandler {
    pub(crate) async fn handle_init_enclave_request(
        &self,
        request: InitEnclaveRequest,
    ) -> Result<InitEnclaveResponse, EnclaveError> {
        info!("Received init enclave request");

        self.context.kms_keys.set_aws_region(request.aws_region.clone()).await;
        self.context.kms_client.init(request.into())?;
        Ok(InitEnclaveResponse::success())
    }

    pub(crate) async fn handle_add_kms_key_request(
        &self,
        request: AddKmsKeyRequest,
    ) -> Result<AddKmsKeyResponse, EnclaveError> {
        info!("Received add kms key request");
        //todo: check key id is valid
        self.context.kms_keys.add_key(request.into()).await;
        Ok(AddKmsKeyResponse::success())
    }

    pub(crate) async fn handle_update_aws_credentials_request(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveError> {
        info!("Received update AWS credentials request");
        self.context.kms_client.update_aws_credentials(request.into())?;
        Ok(UpdateAwsCredentialsResponse::success())
    }
}
