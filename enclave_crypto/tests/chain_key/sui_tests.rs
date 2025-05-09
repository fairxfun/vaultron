use enclave_crypto::{derive_sui_keypair, random_bytes_by_rng};
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey};
use fastcrypto::traits::{KeyPair, Signer, ToFromBytes, VerifyingKey};

#[tokio::test]
pub async fn test_sui_key_pair() {
    for _ in 0..10 {
        let seed = random_bytes_by_rng(32);
        let key_pair = derive_sui_keypair(&seed, None).unwrap();

        // Check lengths
        assert_eq!(key_pair.private_key.len(), 32);
        assert_eq!(key_pair.public_address.len(), 32);

        // Create a Sui keypair from the private key
        let sui_private_key = Ed25519PrivateKey::from_bytes(&key_pair.private_key).unwrap();
        let sui_keypair: Ed25519KeyPair = sui_private_key.into();

        // Test signing and verification
        let message = b"Hello, Sui!";
        let signature = sui_keypair.sign(message);
        assert!(sui_keypair.public().verify(message, &signature).is_ok());
    }
}

#[tokio::test]
pub async fn test_sui_key_pair_with_indices() {
    let seed = random_bytes_by_rng(32);

    // Test different indices
    for index in 0..5 {
        let key_pair = derive_sui_keypair(&seed, Some(index)).unwrap();
        let key_pair2 = derive_sui_keypair(&seed, Some(index)).unwrap();

        // Same index should produce same key pair
        assert_eq!(key_pair.private_key, key_pair2.private_key);
        assert_eq!(key_pair.public_address, key_pair2.public_address);

        // Different indices should produce different key pairs
        if index > 0 {
            let prev_key_pair = derive_sui_keypair(&seed, Some(index - 1)).unwrap();
            assert_ne!(key_pair.private_key, prev_key_pair.private_key);
            assert_ne!(key_pair.public_address, prev_key_pair.public_address);
        }
    }
}

#[tokio::test]
pub async fn test_sui_key_pair_consistency() {
    let seed = random_bytes_by_rng(32);
    let key_pair1 = derive_sui_keypair(&seed, None).unwrap();
    let key_pair2 = derive_sui_keypair(&seed, None).unwrap();

    // Same seed and index should always produce same key pair
    assert_eq!(key_pair1.private_key, key_pair2.private_key);
    assert_eq!(key_pair1.public_address, key_pair2.public_address);

    // Verify the key pair can sign and verify
    let sui_private_key = Ed25519PrivateKey::from_bytes(&key_pair1.private_key).unwrap();
    let sui_keypair: Ed25519KeyPair = sui_private_key.into();

    let message = b"Test message!";
    let signature = sui_keypair.sign(message);
    assert!(sui_keypair.public().verify(message, &signature).is_ok());
}
