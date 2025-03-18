use std::sync::Arc;

use anyhow::Result;
use enclave_agent::{create_enclave_agent, EnclaveAgentCreateOptions, EnclaveAgentTrait};
use enclave_protos::enclave::v1::{
    CreateEnclaveWalletRequest, InitKmstoolRequest, PingRequest, SignatureType, StatusCode,
};
use enclave_utils::address::ethers_address_to_bytes;
use ethers_core::rand::thread_rng;
use ethers_signers::{Signer, Wallet};

use crate::EnclaveAgentTesterError;

pub async fn start_tester() -> Result<(), Box<dyn std::error::Error>> {
    let options = EnclaveAgentCreateOptions::builder().build();
    let agent = create_enclave_agent(options)?;

    test_init_kmstool(agent.clone()).await;
    test_ping(agent.clone()).await;
    test_create_enclave_wallet(agent.clone()).await;
    Ok(())
}

async fn test_init_kmstool(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let (access_key_id, secret_access_key, session_token) =
        get_aws_credentials().await.expect("Get aws credentials failed");
    let request = InitKmstoolRequest::builder()
        .enable_logging(false)
        .aws_region("ap-southeast-1".to_string())
        .aws_access_key_id(access_key_id)
        .aws_secret_access_key(secret_access_key)
        .aws_session_token(session_token)
        .kms_key_id("60b8ce3a-7466-42b7-96a7-a3868f0fd1bf".to_string())
        .build();
    let response = agent.kmstool_init(request).await.expect("Init kmstool failed");
    assert_eq!(response.code, Some(StatusCode::success()));
}

async fn test_ping(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let request = PingRequest::builder().build();
    let response = agent.ping(request).await.expect("Ping failed");
    assert_eq!(response.code, Some(StatusCode::success()));
}

async fn test_create_enclave_wallet(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let mut rng = thread_rng();
    let user_id = vec![1];
    let user_wallet = Wallet::new(&mut rng);
    let user_address = ethers_address_to_bytes(&user_wallet.address());
    let create_message = "create enclave wallet";
    let signature = user_wallet.sign_message(create_message.as_bytes()).await.unwrap();
    let request = CreateEnclaveWalletRequest::builder()
        .user_id(user_id.clone())
        .user_public_key(user_address.clone())
        .signature_type(SignatureType::WalletEth)
        .message(create_message)
        .signature(signature.to_vec())
        .build();
    let response = agent
        .create_enclave_wallet(request)
        .await
        .expect("Create enclave wallet failed");
    assert_eq!(response.code, Some(StatusCode::success()));
    println!("Create enclave wallet response: {:?}", response);
}

async fn get_aws_credentials() -> Result<(String, String, String), EnclaveAgentTesterError> {
    let url = "http://169.254.169.254/latest/meta-data/iam/security-credentials/fairx_enclave_role";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let credentials: serde_json::Value = serde_json::from_str(&body)?;
    let access_key_id = credentials["AccessKeyId"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("AccessKeyId not found".to_string()))?;
    let secret_access_key = credentials["SecretAccessKey"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("SecretAccessKey not found".to_string()))?;
    let session_token = credentials["Token"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("Token not found".to_string()))?;
    Ok((
        access_key_id.to_string(),
        secret_access_key.to_string(),
        session_token.to_string(),
    ))
}
