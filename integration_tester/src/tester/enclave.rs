// use crate::TesterMessageHandler;
use crate::{error::EnclaveTesterError, handler::MessageHandler};
use anyhow::Result;
use enclave_crypto::{random_bytes_by_rng, EciesKeyPair};
use enclave_protos::vaultron::enclave::cluster::v1::{VaultronCreationAttributes, VaultronUserSignatureType};
// use enclave_agent::EnclaveAgentCreateOptions;
use enclave_protos::vaultron::enclave::internal::v1::{InitClusterKeyRequest, InitClusterKeyResponse};
use enclave_protos::vaultron::{
    enclave::attestation::v1::Digest,
    enclave::cluster::v1::{ClusterAttributes, CreateEnclaveVaultronRequest, CreateEnclaveVaultronResponse},
    enclave::internal::v1::{PingRequest, PingResponse},
    enclave::v1::{
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
use vaultron_enclave::recover_multi_chain_wallet;

pub async fn enclave_test(handler: &mut MessageHandler, pcr0: Vec<u8>) -> Result<(), EnclaveTesterError> {
    test_ping(handler).await;

    let cluster_public_key = test_init_cluster_key(handler).await;
    let cluster_attributes = ClusterAttributes::builder()
        .enclave_pcr0(pcr0)
        .cluster_public_key(cluster_public_key)
        .build();
    let iteration_start = Instant::now();
    test_create_enclave_wallet(cluster_attributes, handler).await;
    let iteration_end = Instant::now();
    let duration = iteration_end.duration_since(iteration_start);
    info!(" test create enclave wallet done in {:?}", duration);

    sleep(Duration::from_secs(3)).await;
    Ok(())
}

async fn test_ping(handler: &mut MessageHandler) {
    info!("test ping");
    let ping_request = PingRequest::builder().build();
    let internal_request = EnclaveInternalRequest::builder()
        .request(enclave_internal_request::Request::PingRequest(ping_request.clone()))
        .build();
    let request = EnclaveRequest::builder()
        .request(enclave_request::Request::InternalRequest(internal_request))
        .build();
    let (r, _p, _doc) = handler
        .send_enclave_request::<PingRequest, PingResponse>(request)
        .await
        .map_err(|err| info!("request meet error {:?}", err))
        .expect("Ping failed");
    assert_eq!(r, ping_request);
}

async fn test_init_cluster_key(handler: &mut MessageHandler) -> Vec<u8> {
    info!("test init cluster key");
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
        .send_enclave_request::<InitClusterKeyRequest, InitClusterKeyResponse>(request)
        .await
        .map_err(|err| info!("request meet error {:?}", err))
        .expect("Init cluster key failed");
    assert_eq!(r, init_cluster_key_request);
    assert_ne!(doc.public_key, p.cluster_public_key);
    assert_eq!(doc.digest, Digest::Sha384 as i32);
    assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
    p.cluster_public_key
}

async fn test_create_enclave_wallet(cluster_attributes: ClusterAttributes, handler: &mut MessageHandler) {
    info!("test create enclave wallet");
    let mut rng = thread_rng();
    let user_id = "test_user_id";
    let user_wallet = Wallet::new(&mut rng);
    let user_address = ethers_address_to_bytes(&user_wallet.address());
    let create_message = "create enclave wallet";
    let seed = random_bytes_by_rng(32);
    let ecies_key_pair = EciesKeyPair::from_seed(&seed).unwrap();
    let create_wallet_data = VaultronCreationAttributes::builder()
        .cluster(cluster_attributes)
        .user_id(user_id.to_string())
        .public_key(user_address.clone())
        .recovery_public_key(ecies_key_pair.public_key().to_vec())
        .signature_type(VaultronUserSignatureType::WalletEth as i32)
        .message(create_message)
        .build();
    let request_message = create_wallet_data.encode_to_vec();
    let signature = user_wallet.sign_message(request_message).await.unwrap();
    let wallet_request = CreateEnclaveVaultronRequest::builder()
        .attributes(create_wallet_data)
        .signature(signature.to_vec())
        .build();
    let cluster_request = EnclaveClusterRequest::builder()
        .request(enclave_cluster_request::Request::CreateVaultronRequest(
            wallet_request.clone(),
        ))
        .build();
    let request = EnclaveRequest::builder()
        .request(enclave_request::Request::ClusterRequest(cluster_request))
        .build();
    let (r, p, doc) = handler
        .send_enclave_request::<CreateEnclaveVaultronRequest, CreateEnclaveVaultronResponse>(request)
        .await
        .expect("Create enclave wallet failed");
    assert_eq!(r, wallet_request);
    let user_wallet_seed = ecies_key_pair.decrypt_by_private_key(&p.encrypted_seed).unwrap();
    let user_wallet_seed_str = String::from_utf8(user_wallet_seed).unwrap();
    let user_wallet = recover_multi_chain_wallet(&user_wallet_seed_str, None).unwrap();
    assert_eq!(user_wallet.mnemonic, user_wallet_seed_str);
    assert_eq!(user_wallet.eth_keypair.public_address, p.eth_public_key);
    assert_eq!(user_wallet.solana_keypair.public_address, p.solana_public_key);
    assert_eq!(user_wallet.sui_keypair.public_address, p.sui_public_key.clone());
    assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
    assert_eq!(doc.digest, Digest::Sha384 as i32);
}
