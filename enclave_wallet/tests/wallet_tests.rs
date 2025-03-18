use enclave_utils::hex::encode_hex_with_prefix;
use enclave_wallet::{generate_multi_chain_wallet, recover_multi_chain_wallet};
use ethers_core::types::H160;
use ethers_signers::{Signer, Wallet};
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey};
use fastcrypto::hash::{Blake2b256, HashFunction};
use fastcrypto::traits::{KeyPair as FastCryptoKeyPair, ToFromBytes};
use solana_keypair::Keypair as SolanaKeypair;
use solana_signer::Signer as SolanaSigner;
use std::str;
use sui_types::base_types::SuiAddress;

#[tokio::test]
pub async fn test_generate_multi_chain_wallet() {
    let wallet = generate_multi_chain_wallet().unwrap();

    // Print private keys and addresses for verification
    println!(
        "ETH private key: 0x{}",
        encode_hex_with_prefix(&wallet.eth_keypair.private_key)
    );
    println!(
        "ETH address: 0x{}",
        encode_hex_with_prefix(&wallet.eth_keypair.public_address)
    );

    println!(
        "Solana private key: 0x{}",
        encode_hex_with_prefix(&wallet.solana_keypair.private_key)
    );

    let solana_address_str = str::from_utf8(&wallet.solana_keypair.public_address).unwrap();
    println!("Solana address: {}", solana_address_str);

    println!(
        "SUI private key: 0x{}",
        encode_hex_with_prefix(&wallet.sui_keypair.private_key)
    );
    println!(
        "SUI address (hex): 0x{}",
        encode_hex_with_prefix(&wallet.sui_keypair.public_address)
    );

    let ether_key = Wallet::from_bytes(&wallet.eth_keypair.private_key).unwrap();
    let expected_address = ether_key.address();
    let actual_address = H160::from_slice(&wallet.eth_keypair.public_address);
    assert_eq!(expected_address, actual_address);

    let solana_keypair = SolanaKeypair::from_bytes(&wallet.solana_keypair.private_key).unwrap();
    let solana_pubkey = SolanaSigner::pubkey(&solana_keypair);
    let expected_solana_address = solana_pubkey.to_string().into_bytes();
    assert_eq!(wallet.solana_keypair.public_address, expected_solana_address);

    let sui_private_key = Ed25519PrivateKey::from_bytes(&wallet.sui_keypair.private_key).unwrap();
    let sui_keypair: Ed25519KeyPair = sui_private_key.into();
    let sui_address = ed25519_public_key_to_sui_address(sui_keypair.public());
    assert_eq!(wallet.sui_keypair.public_address, sui_address.to_vec());

    let recovered_wallet = recover_multi_chain_wallet(&wallet.mnemonic, None).unwrap();
    assert_eq!(wallet.mnemonic, recovered_wallet.mnemonic);
    assert_eq!(wallet.eth_keypair, recovered_wallet.eth_keypair);
    assert_eq!(wallet.solana_keypair, recovered_wallet.solana_keypair);
    assert_eq!(wallet.sui_keypair, recovered_wallet.sui_keypair);

    let recovered_wallet_with_index = recover_multi_chain_wallet(&wallet.mnemonic, Some(0)).unwrap();
    assert_eq!(wallet, recovered_wallet_with_index);

    let recovered_wallet_with_index_1_1 = recover_multi_chain_wallet(&wallet.mnemonic, Some(1)).unwrap();
    let recovered_wallet_with_index_1_2 = recover_multi_chain_wallet(&wallet.mnemonic, Some(1)).unwrap();
    assert_eq!(recovered_wallet_with_index_1_1, recovered_wallet_with_index_1_2);

    let recovered_wallet_with_index_1001_1 = recover_multi_chain_wallet(&wallet.mnemonic, Some(1001)).unwrap();
    let recovered_wallet_with_index_1001_2 = recover_multi_chain_wallet(&wallet.mnemonic, Some(1001)).unwrap();
    assert_eq!(recovered_wallet_with_index_1001_1, recovered_wallet_with_index_1001_2);
}

fn ed25519_public_key_to_sui_address(pub_key: &Ed25519PublicKey) -> SuiAddress {
    let scheme_flag = [0x00u8];
    let mut hasher = Blake2b256::new();
    hasher.update(scheme_flag);
    hasher.update(pub_key.as_ref());
    let hash = hasher.finalize();
    SuiAddress::from_bytes(hash.as_ref()).expect("Invalid address bytes")
}
