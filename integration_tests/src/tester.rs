use std::{str, sync::Arc};

use anyhow::Result;
use enclave_agent::{create_enclave_agent, EnclaveAgentCreateOptions, EnclaveAgentTrait};
use enclave_protos::enclave::v1::{
    AddKmsKeyRequest, CreateEnclaveWalletRequest, GetEnclavePcrRequest, InitEnclaveRequest, KmsData, PingRequest,
    SignatureType, StatusCode, UpdateAwsCredentialsRequest,
};
use enclave_utils::{
    address::{ethers_address_to_bytes, string_address_from_bytes},
    hex::encode_hex_with_prefix,
};
use ethers_core::rand::thread_rng;
use ethers_signers::{Signer, Wallet};
use std::time::Instant;
use tokio::time::{sleep, Duration};

pub async fn start_test() -> Result<(), Box<dyn std::error::Error>> {
    let options = EnclaveAgentCreateOptions::builder().build();
    let agent = create_enclave_agent(options)?;

    test_init_kmstool(agent.clone()).await;
    test_update_aws_credentials(agent.clone()).await;
    test_get_enclave_pcr(agent.clone()).await;
    test_add_kms_key(agent.clone()).await;
    test_ping(agent.clone()).await;
    for i in 0..100000 {
        let iteration_start = Instant::now();
        test_create_enclave_wallet(agent.clone()).await;
        let iteration_end = Instant::now();
        let duration = iteration_end.duration_since(iteration_start);
        println!(" test {} done in {:?}", i, duration);

        if i % 1000 == 0 {
            test_update_aws_credentials(agent.clone()).await;
        }

        sleep(Duration::from_secs(3)).await;
    }
    Ok(())
}

async fn test_init_kmstool(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let request = InitEnclaveRequest::builder()
        .enable_logging(true)
        .aws_region("ap-southeast-1".to_string())
        .build();
    let response = agent.init_enclave(request).await.expect("Init kmstool failed");
    assert_eq!(response.code, Some(StatusCode::success()));
}

async fn test_add_kms_key(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let request = AddKmsKeyRequest::builder()
        .kms_key_id("60b8ce3a-7466-42b7-96a7-a3868f0fd1bf".to_string())
        .build();
    let response = agent.add_kms_key(request).await.expect("Add kms key failed");
    assert_eq!(response.code, Some(StatusCode::success()));
}

async fn test_update_aws_credentials(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let (access_key_id, secret_access_key, session_token) = get_aws_credentials().await;
    let request = UpdateAwsCredentialsRequest::builder()
        .aws_access_key_id(access_key_id)
        .aws_secret_access_key(secret_access_key)
        .aws_session_token(session_token)
        .build();
    let response = agent
        .update_aws_credentials(request)
        .await
        .expect("Update aws credentials failed");
    assert_eq!(response.code, Some(StatusCode::success()));
    println!("Update aws credentials success");
}

async fn test_get_enclave_pcr(agent: Arc<Box<dyn EnclaveAgentTrait>>) {
    let request = GetEnclavePcrRequest::builder().build();
    let response = agent.get_enclave_pcr(request).await.expect("Get enclave pcr failed");
    assert_eq!(response.code, Some(StatusCode::success()));
    for (index, pcr) in response.pcrs.iter().enumerate() {
        println!("index {} pcr: {}", index, pcr);
    }
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
        .kms_data(
            KmsData::builder()
                .kms_key_id("60b8ce3a-7466-42b7-96a7-a3868f0fd1bf".to_string())
                .build(),
        )
        .build();
    let response = agent
        .create_enclave_wallet(request)
        .await
        .expect("Create enclave wallet failed");
    assert_eq!(response.code, Some(StatusCode::success()));
    println!(
        "eth account address: {:?}",
        string_address_from_bytes(&response.eth_public_key)
    );
    println!(
        "solana account address: 0x{:?}",
        str::from_utf8(&response.solana_public_key).unwrap()
    );
    println!(
        "sui account address: {:?}",
        encode_hex_with_prefix(&response.sui_public_key)
    );
}

async fn get_aws_credentials() -> (String, String, String) {
    let url = "http://169.254.169.254/latest/meta-data/iam/security-credentials/fairx_enclave_role";
    let response = reqwest::get(url).await.expect("Get aws credentials failed");
    let body = response.text().await.expect("Get aws credentials failed");
    let credentials: serde_json::Value = serde_json::from_str(&body).expect("Parse aws credentials failed");
    let access_key_id = credentials["AccessKeyId"].as_str().expect("AccessKeyId not found");
    let secret_access_key = credentials["SecretAccessKey"]
        .as_str()
        .expect("SecretAccessKey not found");
    let session_token = credentials["Token"].as_str().expect("Token not found");
    (
        access_key_id.to_string(),
        secret_access_key.to_string(),
        session_token.to_string(),
    )
}
