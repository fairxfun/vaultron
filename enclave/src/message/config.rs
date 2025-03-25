use super::MessageHandler;
use crate::{
    common::EnclaveError,
    data::get_pcr_from_attestation_doc,
    kms_key::{KmsKeyPolicies, KmsKeyPolicy, DEFAULT_POLICY_NAME},
};
use enclave_kmstool::{KmstoolGetKeyPolicyParams, KmstoolListKeyPoliciesParams};
use enclave_protos::enclave::v1::{
    AddKmsKeyRequest, AddKmsKeyResponse, GetEnclavePcrRequest, GetEnclavePcrResponse, InitEnclaveRequest,
    InitEnclaveResponse, StatusCode, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use log::info;
use std::convert::TryFrom;

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

    pub(crate) async fn handle_update_aws_credentials_request(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveError> {
        info!("Received update AWS credentials request");
        self.context.kms_client.update_aws_credentials(request.into())?;
        self.load_enclave_pcrs().await?;
        Ok(UpdateAwsCredentialsResponse::success())
    }

    pub(crate) async fn handle_get_enclave_pcr_request(
        &self,
        _request: GetEnclavePcrRequest,
    ) -> Result<GetEnclavePcrResponse, EnclaveError> {
        info!("Received get enclave pcr request");
        let pcrs = self.context.enclave_data.get_pcrs().await;
        Ok(GetEnclavePcrResponse::builder()
            .code(StatusCode::builder().success(true).build())
            .pcrs(pcrs)
            .build())
    }

    pub(crate) async fn handle_add_kms_key_request(
        &self,
        request: AddKmsKeyRequest,
    ) -> Result<AddKmsKeyResponse, EnclaveError> {
        info!("Received add kms key request");
        self.verify_key_id(&request).await?;
        self.context.kms_keys.add_key(request.into()).await;
        Ok(AddKmsKeyResponse::success())
    }

    async fn load_enclave_pcrs(&self) -> Result<(), EnclaveError> {
        if !self.context.enclave_data.is_initialized().await {
            let attestation = self.context.kms_client.get_attestation_document()?;
            let pcrs = get_pcr_from_attestation_doc(&attestation.attestation_document)?;
            self.context.enclave_data.set_pcrs(pcrs).await;
        }
        Ok(())
    }

    async fn verify_key_id(&self, request: &AddKmsKeyRequest) -> Result<(), EnclaveError> {
        let params = KmstoolListKeyPoliciesParams {
            key_id: request.kms_key_id.clone(),
            limit: None,
            marker: None,
        };
        let key_policies = self.context.kms_client.list_key_policies(params)?;
        let key_policies = KmsKeyPolicies::try_from(key_policies)?;
        if !key_policies.verify() {
            return Err(EnclaveError::InvalidKeyPoliciesError);
        }

        let params = KmstoolGetKeyPolicyParams {
            key_id: request.kms_key_id.clone(),
            policy_name: DEFAULT_POLICY_NAME.to_string(),
        };
        let key_policy = self.context.kms_client.get_key_policy(params)?;
        let key_policy = KmsKeyPolicy::try_from(key_policy)?;
        let pcr0 = self.context.enclave_data.get_pcr0().await;
        if !key_policy.verify(pcr0) {
            return Err(EnclaveError::InvalidKeyPolicyError);
        }
        Ok(())
    }
}
