// use crate::TesterMessageHandler;
use crate::{trace::integration_trace_init, EnclaveTesterError};
use anyhow::Result;
// use enclave_agent::EnclaveAgentCreateOptions;
// use enclave_protos::vaultron::enclave::internal::v1::{InitClusterKeyRequest, InitClusterKeyResponse};
// use enclave_protos::vaultron::{
//     enclave::attestation::v1::Digest,
//     enclave::cluster::v1::{
//         ClusterAttributes, CreateUserWalletParams, CreateUserWalletRequest, CreateUserWalletResponse, SignatureType,
//     },
//     enclave::internal::v1::{PingRequest, PingResponse},
//     enclave::v1::{
//         enclave_cluster_request, enclave_internal_request, enclave_request, EnclaveClusterRequest,
//         EnclaveInternalRequest, EnclaveRequest,
//     },
// };
// use enclave_utils::address::ethers_address_to_bytes;
// use ethers_core::rand::thread_rng;
// use ethers_signers::{Signer, Wallet};
// use log::info;
// use prost::Message;
// use service_discovery::{
//     VaultronServiceQuerier, VaultronServiceQuerierConfig, VaultronServiceRegister, VaultronServiceRegisterConfig,
// };
// use std::collections::HashMap;
// use std::time::Instant;
// use tokio::time::{sleep, Duration};

pub async fn start_test(_pcr0: Vec<u8>) -> Result<(), EnclaveTesterError> {
    integration_trace_init("info")?;

    // test_service_discovery().await;

    // let options = EnclaveAgentCreateOptions::builder().build();
    // let mut handler = TesterMessageHandler::new(pcr0.clone(), options).await?;
    // test_ping(&mut handler).await;
    // let cluster_public_key = test_init_cluster_key(&mut handler).await;

    // let cluster_attributes = ClusterAttributes::builder()
    //     .enclave_pcr0(pcr0)
    //     .cluster_public_key(cluster_public_key)
    //     .build();
    // let iteration_start = Instant::now();
    // test_create_enclave_wallet(cluster_attributes, &mut handler).await;
    // let iteration_end = Instant::now();
    // let duration = iteration_end.duration_since(iteration_start);
    // println!(" test create enclave wallet done in {:?}", duration);
    // sleep(Duration::from_secs(3)).await;
    Ok(())
}

// async fn test_ping(handler: &mut TesterMessageHandler) {
//     let ping_request = PingRequest::builder().build();
//     let internal_request = EnclaveInternalRequest::builder()
//         .request(enclave_internal_request::Request::PingRequest(ping_request.clone()))
//         .build();
//     let request = EnclaveRequest::builder()
//         .request(enclave_request::Request::InternalRequest(internal_request))
//         .build();
//     let (r, _p, _doc) = handler
//         .send_request::<PingRequest, PingResponse>(&request)
//         .await
//         .map_err(|err| info!("request meet error {:?}", err))
//         .expect("Ping failed");
//     assert_eq!(r, ping_request);
// }

// async fn test_init_cluster_key(handler: &mut TesterMessageHandler) -> Vec<u8> {
//     let init_cluster_key_request = InitClusterKeyRequest::builder().build();
//     let internal_request = EnclaveInternalRequest::builder()
//         .request(enclave_internal_request::Request::InitClusterKeyRequest(
//             init_cluster_key_request.clone(),
//         ))
//         .build();
//     let request = EnclaveRequest::builder()
//         .request(enclave_request::Request::InternalRequest(internal_request))
//         .build();
//     let (r, p, doc) = handler
//         .send_request::<InitClusterKeyRequest, InitClusterKeyResponse>(&request)
//         .await
//         .map_err(|err| info!("request meet error {:?}", err))
//         .expect("Init cluster key failed");
//     assert_eq!(r, init_cluster_key_request);
//     assert_ne!(doc.public_key, p.cluster_public_key);
//     assert_eq!(doc.digest, Digest::Sha384 as i32);
//     assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
//     p.cluster_public_key
// }

// async fn test_create_enclave_wallet(cluster_attributes: ClusterAttributes, handler: &mut TesterMessageHandler) {
//     let mut rng = thread_rng();
//     let user_id = vec![1];
//     let user_wallet = Wallet::new(&mut rng);
//     let user_address = ethers_address_to_bytes(&user_wallet.address());
//     let create_message = "create enclave wallet";
//     let create_wallet_data = CreateUserWalletParams::builder()
//         .attributes(cluster_attributes)
//         .user_id(user_id.clone())
//         .user_public_key(user_address.clone())
//         .signature_type(SignatureType::WalletEth)
//         .message(create_message)
//         .build();
//     let request_message = create_wallet_data.encode_to_vec();
//     let signature = user_wallet.sign_message(request_message).await.unwrap();
//     let wallet_request = CreateUserWalletRequest::builder()
//         .params(create_wallet_data)
//         .signature(signature.to_vec())
//         .build();
//     let cluster_request = EnclaveClusterRequest::builder()
//         .request(enclave_cluster_request::Request::CreateUserWalletRequest(
//             wallet_request.clone(),
//         ))
//         .build();
//     let request = EnclaveRequest::builder()
//         .request(enclave_request::Request::ClusterRequest(cluster_request))
//         .build();
//     let (r, p, doc) = handler
//         .send_request::<CreateUserWalletRequest, CreateUserWalletResponse>(&request)
//         .await
//         .expect("Create enclave wallet failed");
//     assert_eq!(r, wallet_request);
//     assert!(is_valid_eth_address(&p.eth_public_key));
//     assert!(is_valid_solana_address(&p.solana_public_key));
//     assert!(is_valid_sui_address(&p.sui_public_key));
//     assert_eq!(doc.pcrs.get(&0).unwrap(), &handler.pcr0);
//     assert_eq!(doc.digest, Digest::Sha384 as i32);
// }

// async fn test_service_discovery() {
//     let service_id = "srv-33eez4kuqhljwa2o".to_string();
//     let instance_id = "test-enclave-agent".to_string();
//     let region = "".to_string();
//     let mut attributes = HashMap::new();
//     attributes.insert("version".to_string(), "1.0.0".to_string());
//     let register_config = VaultronServiceRegisterConfig::builder()
//         .region(region.clone())
//         .service_id(service_id.clone())
//         .instance_id(instance_id.clone())
//         .attributes(attributes)
//         .build();
//     let register = VaultronServiceRegister::new(register_config).await;
//     register.register_service().await.unwrap();

//     sleep(Duration::from_secs(5)).await;

//     let querier_config = VaultronServiceQuerierConfig::builder().region(region.clone()).build();
//     let querier = VaultronServiceQuerier::new(querier_config).await;

//     let instances = querier.list_instances(service_id.clone()).await.unwrap();
//     println!("instances: {:?}", instances);
//     let instance = querier
//         .get_instance(service_id.clone(), instance_id.clone())
//         .await
//         .unwrap();
//     println!("instance: {:?}", instance);
//     let instance_ids = instances.iter().map(|i| i.id.clone()).collect();
//     let instance_health_status = querier
//         .get_instance_health_status(service_id.clone(), instance_ids)
//         .await
//         .unwrap();
//     println!("instance_health_status: {:?}", instance_health_status);
// }

// fn is_valid_eth_address(address: &[u8]) -> bool {
//     address.len() == 20
// }

// fn is_valid_solana_address(address: &[u8]) -> bool {
//     address.len() == 44
// }

// fn is_valid_sui_address(address: &[u8]) -> bool {
//     address.len() == 32
// }
