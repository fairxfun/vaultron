use enclave_crypto::{derive_solana_keypair, random_bytes_by_rng};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[tokio::test]
pub async fn test_solana_key_pair() {
    for _ in 0..10 {
        let seed = random_bytes_by_rng(32);
        let key_pair = derive_solana_keypair(&seed, None).unwrap();

        // Check lengths
        assert_eq!(key_pair.private_key.len(), 64);
        assert!(key_pair.public_address.len() == 44 || key_pair.public_address.len() == 43);

        // Create a Solana keypair from the private key
        let mut keypair_bytes = [0u8; 64];
        keypair_bytes.copy_from_slice(&key_pair.private_key);
        let solana_keypair = Keypair::from_bytes(&keypair_bytes).unwrap();

        // Verify the public key matches
        assert_eq!(
            solana_keypair.pubkey().to_string().into_bytes(),
            key_pair.public_address
        );

        // Test signing and verification
        let message = b"Hello, Solana!";
        let signature = solana_keypair.sign_message(message);

        // Verify the signature
        assert!(signature.verify(&solana_keypair.pubkey().to_bytes(), message));
    }
}

#[tokio::test]
pub async fn test_solana_key_pair_with_indices() {
    let seed = random_bytes_by_rng(32);

    // Test different indices
    for index in 0..5 {
        let key_pair = derive_solana_keypair(&seed, Some(index)).unwrap();
        let key_pair2 = derive_solana_keypair(&seed, Some(index)).unwrap();

        // Same index should produce same key pair
        assert_eq!(key_pair.private_key, key_pair2.private_key);
        assert_eq!(key_pair.public_address, key_pair2.public_address);

        // Different indices should produce different key pairs
        if index > 0 {
            let prev_key_pair = derive_solana_keypair(&seed, Some(index - 1)).unwrap();
            assert_ne!(key_pair.private_key, prev_key_pair.private_key);
            assert_ne!(key_pair.public_address, prev_key_pair.public_address);
        }
    }
}

#[tokio::test]
pub async fn test_solana_key_pair_consistency() {
    let seed = random_bytes_by_rng(32);
    let key_pair1 = derive_solana_keypair(&seed, None).unwrap();
    let key_pair2 = derive_solana_keypair(&seed, None).unwrap();

    // Same seed and index should always produce same key pair
    assert_eq!(key_pair1.private_key, key_pair2.private_key);
    assert_eq!(key_pair1.public_address, key_pair2.public_address);

    // Verify the key pair can sign and verify
    let mut keypair_bytes = [0u8; 64];
    keypair_bytes.copy_from_slice(&key_pair1.private_key);
    let solana_keypair = Keypair::from_bytes(&keypair_bytes).unwrap();

    let message = b"Test message";
    let signature = solana_keypair.sign_message(message);
    assert!(signature.verify(&solana_keypair.pubkey().to_bytes(), message));
}
