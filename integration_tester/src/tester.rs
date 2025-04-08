use crate::TesterMessageHandler;
use crate::{trace::integration_trace_init, EnclaveTesterError};
use anyhow::Result;
use enclave_agent::EnclaveAgentCreateOptions;
use enclave_protos::vaultron::internal::v1::{InitClusterKeyRequest, InitClusterKeyResponse};
use enclave_protos::vaultron::{
    attestation::v1::Digest,
    cluster::v1::{
        ClusterData, CreateUserWalletData, CreateUserWalletRequest, CreateUserWalletResponse, SignatureType,
    },
    internal::v1::{PingRequest, PingResponse},
    v1::{
        enclave_cluster_request, enclave_internal_request, enclave_request, EnclaveClusterRequest,
        EnclaveInternalRequest, EnclaveRequest,
    },
};
use enclave_utils::address::ethers_address_to_bytes;
use ethers_core::rand::thread_rng;
use ethers_signers::{Signer, Wallet};
use log::info;
use prost::Message;
use std::time::Instant;
use tokio::time::{sleep, Duration};

pub async fn start_test(pcr0: Vec<u8>) -> Result<(), EnclaveTesterError> {
    integration_trace_init("info")?;
    let options = EnclaveAgentCreateOptions::builder().build();
    let mut handler = TesterMessageHandler::new(pcr0.clone(), options).await?;
    test_ping(&mut handler).await;
    let cluster_public_key = test_init_cluster_key(&mut handler).await;

    let cluster_data = ClusterData::builder()
        .enclave_pcr0(pcr0)
        .cluster_public_key(cluster_public_key)
        .build();
    let iteration_start = Instant::now();
    test_create_enclave_wallet(cluster_data, &mut handler).await;
    let iteration_end = Instant::now();
    let duration = iteration_end.duration_since(iteration_start);
    println!(" test create enclave wallet done in {:?}", duration);
    sleep(Duration::from_secs(3)).await;
    Ok(())
}

async fn test_ping(handler: &mut TesterMessageHandler) {
    let ping_request = PingRequest::builder().build();
    let internal_request = EnclaveInternalRequest::builder()
        .request(enclave_internal_request::Request::PingRequest(ping_request.clone()))
        .build();
    let request = EnclaveRequest::builder()
        .request(enclave_request::Request::InternalRequest(internal_request))
        .build();
    let (r, p, doc) = handler
        .send_request::<PingRequest, PingResponse>(&request)
        .await
        .map_err(|err| info!("request meet error {:?}", err))
        .expect("Ping failed");
    let enclave_data = p.enclave_data.unwrap();
    assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
    assert_eq!(doc.digest, Digest::Sha384 as i32);
    assert_eq!(doc.public_key, enclave_data.enclave_public_key);
    assert_eq!(r, ping_request);
}

async fn test_init_cluster_key(handler: &mut TesterMessageHandler) -> Vec<u8> {
    let init_cluster_key_request = InitClusterKeyRequest::builder().build();
    let internal_request = EnclaveInternalRequest::builder()
        .request(enclave_internal_request::Request::InitClusterKeyRequest(
            init_cluster_key_request.clone(),
        ))
        .build();
    let request = EnclaveRequest::builder()
        .request(enclave_request::Request::InternalRequest(internal_request))
        .build();
    let (r, p, doc) = handler
        .send_request::<InitClusterKeyRequest, InitClusterKeyResponse>(&request)
        .await
        .map_err(|err| info!("request meet error {:?}", err))
        .expect("Init cluster key failed");
    assert_eq!(r, init_cluster_key_request);
    assert_ne!(doc.public_key, p.cluster_public_key);
    assert_eq!(doc.digest, Digest::Sha384 as i32);
    assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
    p.cluster_public_key
}

async fn test_create_enclave_wallet(cluster_data: ClusterData, handler: &mut TesterMessageHandler) {
    let mut rng = thread_rng();
    let user_id = vec![1];
    let user_wallet = Wallet::new(&mut rng);
    let user_address = ethers_address_to_bytes(&user_wallet.address());
    let create_message = "create enclave wallet";
    let create_wallet_data = CreateUserWalletData::builder()
        .cluster_data(cluster_data)
        .user_id(user_id.clone())
        .user_public_key(user_address.clone())
        .signature_type(SignatureType::WalletEth)
        .message(create_message)
        .build();
    let request_message = create_wallet_data.encode_to_vec();
    let signature = user_wallet.sign_message(request_message).await.unwrap();
    let wallet_request = CreateUserWalletRequest::builder()
        .data(create_wallet_data)
        .signature(signature.to_vec())
        .build();
    let cluster_request = EnclaveClusterRequest::builder()
        .request(enclave_cluster_request::Request::CreateUserWalletRequest(
            wallet_request.clone(),
        ))
        .build();
    let request = EnclaveRequest::builder()
        .request(enclave_request::Request::ClusterRequest(cluster_request))
        .build();
    let (r, p, doc) = handler
        .send_request::<CreateUserWalletRequest, CreateUserWalletResponse>(&request)
        .await
        .expect("Create enclave wallet failed");
    assert_eq!(r, wallet_request);
    assert!(is_valid_eth_address(&p.eth_public_key));
    assert!(is_valid_solana_address(&p.solana_public_key));
    assert!(is_valid_sui_address(&p.sui_public_key));
    assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
    assert_eq!(doc.digest, Digest::Sha384 as i32);
}

fn is_valid_eth_address(address: &[u8]) -> bool {
    address.len() == 20
}

fn is_valid_solana_address(address: &[u8]) -> bool {
    address.len() == 44
}

fn is_valid_sui_address(address: &[u8]) -> bool {
    address.len() == 32
}
