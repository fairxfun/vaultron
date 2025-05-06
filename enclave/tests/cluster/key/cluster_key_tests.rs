use enclave_crypto::{random_bytes_by_rng, AesGcmData};
use vaultron_enclave::EnclaveClusterKeys;

#[tokio::test]
async fn test_cluster_key() {
    let seed = random_bytes_by_rng(32);
    let cluster_key = EnclaveClusterKeys::new(&seed).unwrap();

    // Test public key generation
    let public_key = cluster_key.get_cluster_public_key();
    assert!(!public_key.is_empty());

    // Test encryption and decryption
    let test_data = b"Test message for encryption";
    let encrypted_data = cluster_key.encrypt(test_data).unwrap();
    assert_ne!(encrypted_data, test_data);

    // Test seed encryption with public key
    let encrypted_seed = cluster_key.encrypt_seed(public_key.clone()).unwrap();
    assert_ne!(encrypted_seed, seed);
}

#[tokio::test]
async fn test_cluster_key_consistency() {
    let seed = random_bytes_by_rng(32);
    let cluster_key1 = EnclaveClusterKeys::new(&seed).unwrap();
    let cluster_key2 = EnclaveClusterKeys::new(&seed).unwrap();

    // Same seed should produce same public key
    assert_eq!(
        cluster_key1.get_cluster_public_key(),
        cluster_key2.get_cluster_public_key()
    );

    // Test encryption consistency
    let test_data = b"Test message";
    let encrypted1 = cluster_key1.encrypt(test_data).unwrap();
    let gcm_data1 = AesGcmData::deserialize(&encrypted1).unwrap();
    let encrypted2 = cluster_key2.encrypt(test_data).unwrap();
    let gcm_data2 = AesGcmData::deserialize(&encrypted2).unwrap();
    assert_ne!(gcm_data1.nonce, gcm_data2.nonce);
    assert_ne!(gcm_data1.encrypted_data, gcm_data2.encrypted_data);

    let decrypted1 = cluster_key1.decrypt(&encrypted1).unwrap();
    let decrypted2 = cluster_key2.decrypt(&encrypted2).unwrap();
    assert_eq!(decrypted1, test_data);
    assert_eq!(decrypted2, test_data);
}

#[tokio::test]
async fn test_cluster_key_different_seeds() {
    let seed1 = random_bytes_by_rng(32);
    let seed2 = random_bytes_by_rng(32);
    let cluster_key1 = EnclaveClusterKeys::new(&seed1).unwrap();
    let cluster_key2 = EnclaveClusterKeys::new(&seed2).unwrap();

    // Different seeds should produce different public keys
    assert_ne!(
        cluster_key1.get_cluster_public_key(),
        cluster_key2.get_cluster_public_key()
    );

    // Test encryption with different keys
    let test_data = b"Test message";
    let encrypted1 = cluster_key1.encrypt(test_data).unwrap();
    let encrypted2 = cluster_key2.encrypt(test_data).unwrap();
    assert_ne!(encrypted1, encrypted2);
}

#[tokio::test]
async fn test_cluster_key_encryption() {
    let seed = random_bytes_by_rng(32);
    let cluster_key = EnclaveClusterKeys::new(&seed).unwrap();

    // Test encryption with different data lengths
    let test_cases = vec![
        b"Short message".to_vec(),
        b"Medium length message for testing".to_vec(),
        vec![0u8; 1024],        // 1KB of zeros
        vec![0u8; 1024 * 1024], // 1MB of zeros
    ];

    for test_data in test_cases {
        let encrypted = cluster_key.encrypt(&test_data).unwrap();
        assert_ne!(encrypted, test_data);
        assert!(!encrypted.is_empty());
    }
}

#[tokio::test]
async fn test_cluster_key_seed() {
    let seed = random_bytes_by_rng(32);
    let cluster_key = EnclaveClusterKeys::new(&seed).unwrap();

    // Test public key generation
    let public_key = cluster_key.get_cluster_public_key();
    assert!(!public_key.is_empty());

    // Test seed encryption with public key
    let encrypted_seed = cluster_key.encrypt_seed(public_key.clone()).unwrap();
    assert_ne!(encrypted_seed, seed);
}

#[tokio::test]
async fn test_cluster_key_seed_consistency() {
    let seed = random_bytes_by_rng(32);
    let cluster_key1 = EnclaveClusterKeys::new(&seed).unwrap();
    let cluster_key2 = EnclaveClusterKeys::new(&seed).unwrap();

    // Same seed should produce same public key
    assert_eq!(
        cluster_key1.get_cluster_public_key(),
        cluster_key2.get_cluster_public_key()
    );

    // Test seed encryption consistency
    let public_key = cluster_key1.get_cluster_public_key();
    let encrypted_seed1 = cluster_key1.encrypt_seed(public_key.clone()).unwrap();
    let encrypted_seed2 = cluster_key2.encrypt_seed(public_key).unwrap();
    assert_ne!(encrypted_seed1, encrypted_seed2);

    let decrypted1 = cluster_key1.decrypt_by_private_key(&encrypted_seed1).unwrap();
    let decrypted2 = cluster_key2.decrypt_by_private_key(&encrypted_seed2).unwrap();
    assert_eq!(decrypted1, seed);
    assert_eq!(decrypted2, seed);
}

#[tokio::test]
async fn test_cluster_key_seed_different() {
    let seed1 = random_bytes_by_rng(32);
    let seed2 = random_bytes_by_rng(32);
    let cluster_key1 = EnclaveClusterKeys::new(&seed1).unwrap();
    let cluster_key2 = EnclaveClusterKeys::new(&seed2).unwrap();

    // Different seeds should produce different public keys
    assert_ne!(
        cluster_key1.get_cluster_public_key(),
        cluster_key2.get_cluster_public_key()
    );

    // Test seed encryption with different keys
    let public_key1 = cluster_key1.get_cluster_public_key();
    let public_key2 = cluster_key2.get_cluster_public_key();
    let encrypted_seed1 = cluster_key1.encrypt_seed(public_key1).unwrap();
    let encrypted_seed2 = cluster_key2.encrypt_seed(public_key2).unwrap();
    assert_ne!(encrypted_seed1, encrypted_seed2);
}
