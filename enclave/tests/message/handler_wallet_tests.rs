use crate::enclave_mock::MockKmstoolClient;
use enclave_kmstool::KmsEncryptResponse;
use enclave_kmstool::KmsToolTrait;
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, EnclaveRequest,
    EnclaveResponse, SignatureType, StatusCode,
};
use enclave_utils::address::ethers_address_to_bytes;
use enclave_vsock::VsockMessageHandlerTrait;
use enclave_wallet::recover_multi_chain_wallet;
use enclave_wallet::MultiChainWallet;
use ethers_core::rand::thread_rng;
use ethers_signers::{Signer, Wallet};
use prost::Message;
use std::sync::Arc;
use vaultron_enclave::common::EnclaveConfig;
use vaultron_enclave::data::KmsAccountMnemonicPair;
use vaultron_enclave::data::{KmsAccountEthPair, KmsAccountSolanaPair, KmsAccountSuiPair};
use vaultron_enclave::message::MessageHandler;
use vaultron_enclave::server::EnclaveServerContext;

#[tokio::test]
pub async fn test_enclave_eth_user_create_enclave_wallet_request() {
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
    let mut kmstool_client = MockKmstoolClient::new();
    kmstool_client.expect_encrypt().times(4).returning(|request| {
        let response = KmsEncryptResponse::builder()
            .ciphertext(bs64::encode(&request.plaintext).as_bytes().to_vec())
            .build();
        Ok(response)
    });
    let kmstool_client = Arc::new(Box::new(kmstool_client) as Box<dyn KmsToolTrait>);
    let config = Arc::new(EnclaveConfig::default());
    let context = Arc::new(
        EnclaveServerContext::builder()
            .config(config)
            .kms_client(kmstool_client)
            .build(),
    );
    let handler = MessageHandler::builder().context(context).build();
    let request = EnclaveRequest::builder()
        .request(Some(enclave_request::Request::CreateEnclaveWalletRequest(request)))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: CreateEnclaveWalletResponse = match response.response {
        Some(enclave_response::Response::CreateEnclaveWalletResponse(response)) => response,
        _ => panic!("Invalid response"),
    };

    assert_eq!(response.code, Some(StatusCode::success()));
    let mnemonic_encrypted_data = bs64::decode(&response.wallet_encrypted_data).unwrap();
    let mnemonic_pair = KmsAccountMnemonicPair::from_bytes(&mnemonic_encrypted_data).unwrap();
    let eth_encrypted_data = bs64::decode(&response.eth_encrypted_data).unwrap();
    let eth_keypair = KmsAccountEthPair::from_bytes(&eth_encrypted_data).unwrap();
    let solana_encrypted_data = bs64::decode(&response.solana_encrypted_data).unwrap();
    let solana_keypair = KmsAccountSolanaPair::from_bytes(&solana_encrypted_data).unwrap();
    let sui_encrypted_data = bs64::decode(&response.sui_encrypted_data).unwrap();
    let sui_keypair = KmsAccountSuiPair::from_bytes(&sui_encrypted_data).unwrap();

    assert_eq!(mnemonic_pair.user_id, user_id);
    assert_eq!(eth_keypair.user_id, user_id);
    assert_eq!(solana_keypair.user_id, user_id);
    assert_eq!(sui_keypair.user_id, user_id);

    assert_eq!(mnemonic_pair.user_public_key, user_address);
    assert_eq!(eth_keypair.user_public_key, user_address);
    assert_eq!(solana_keypair.user_public_key, user_address);
    assert_eq!(sui_keypair.user_public_key, user_address);

    let multi_chain_wallet: MultiChainWallet = recover_multi_chain_wallet(&mnemonic_pair.mnemonic, None).unwrap();
    assert_eq!(multi_chain_wallet.mnemonic, mnemonic_pair.mnemonic);
    assert_eq!(multi_chain_wallet.eth_keypair.private_key, eth_keypair.eth_private_key);
    assert_eq!(
        multi_chain_wallet.solana_keypair.private_key,
        solana_keypair.solana_private_key
    );
    assert_eq!(multi_chain_wallet.sui_keypair.private_key, sui_keypair.sui_private_key);

    assert_eq!(response.eth_public_key, multi_chain_wallet.eth_keypair.public_address);
    assert_eq!(
        response.solana_public_key,
        multi_chain_wallet.solana_keypair.public_address
    );
    assert_eq!(response.sui_public_key, multi_chain_wallet.sui_keypair.public_address);
}
