use crate::enclave_mock::MockKmstoolClient;
use enclave_kmstool::{KmsToolTrait, KmstoolInitParams};
use enclave_kmstool::{KmstoolGetAttestationDocumentResult, KmstoolUpdateAwsCredentialsParams};
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, EnclaveRequest, EnclaveResponse, InitEnclaveRequest, InitEnclaveResponse,
    StatusCode, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use enclave_utils::hex::decode_hex;
use enclave_vsock::VsockMessageHandlerTrait;
use mockall::predicate;
use prost::Message;
use std::sync::Arc;
use vaultron_enclave::common::EnclaveConfig;
use vaultron_enclave::data::{EnclaveData, EnclaveKmsData};
use vaultron_enclave::message::MessageHandler;
use vaultron_enclave::server::EnclaveServerContext;

#[tokio::test]
pub async fn test_enclave_init_enclave_request() {
    let request = InitEnclaveRequest::builder()
        .enable_logging(true)
        .proxy_port(2)
        .aws_region("us-east-1".to_string())
        .build();
    let kms_init_request: KmstoolInitParams = request.clone().into();
    let mut kmstool_client = MockKmstoolClient::new();
    kmstool_client
        .expect_init()
        .with(predicate::eq(kms_init_request))
        .returning(|_| Ok(()));
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
    let handler = MessageHandler::builder().context(context).build();
    let request = EnclaveRequest::builder()
        .request(Some(enclave_request::Request::InitEnclaveRequest(request)))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: InitEnclaveResponse = match response.response {
        Some(enclave_response::Response::InitEnclaveResponse(response)) => response,
        _ => panic!("Invalid response"),
    };
    assert_eq!(response.code, Some(StatusCode::success()));
}

#[tokio::test]
pub async fn test_enclave_update_aws_credentials_request() {
    let request = UpdateAwsCredentialsRequest::builder()
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .build();
    let kms_update_aws_credentials_request = KmstoolUpdateAwsCredentialsParams::builder()
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .build();
    let mut kmstool_client = MockKmstoolClient::new();
    kmstool_client
        .expect_update_aws_credentials()
        .with(predicate::eq(kms_update_aws_credentials_request))
        .returning(|_| Ok(()));
    let attestation_data = include_bytes!("../test_files/enclave/attestation_doc");
    let attestation_data = String::from_utf8(attestation_data.to_vec()).unwrap();
    let attestation_doc = decode_hex(&attestation_data).unwrap();
    kmstool_client.expect_get_attestation_document().returning(move || {
        Ok(KmstoolGetAttestationDocumentResult::builder()
            .attestation_document(attestation_doc.clone())
            .build())
    });
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
    let handler = MessageHandler::builder().context(context).build();
    let request = EnclaveRequest::builder()
        .request(Some(enclave_request::Request::UpdateAwsCredentialsRequest(request)))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: UpdateAwsCredentialsResponse = match response.response {
        Some(enclave_response::Response::UpdateAwsCredentialsResponse(response)) => response,
        _ => panic!("Invalid response"),
    };
    assert_eq!(response.code, Some(StatusCode::success()));
}
