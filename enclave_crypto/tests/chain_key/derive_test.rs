use enclave_crypto::{derive_key_from_path, generate_mnemonic, mnemonic_to_seed};

#[test]
fn test_generate_mnemonic() {
    // Test generating 12-word mnemonic
    let mnemonic = generate_mnemonic(12).unwrap();
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    assert_eq!(words.len(), 12);

    // Test generating 24-word mnemonic
    let mnemonic = generate_mnemonic(24).unwrap();
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    assert_eq!(words.len(), 24);

    // Test invalid word count
    assert!(generate_mnemonic(13).is_err());
}

#[test]
fn test_mnemonic_to_seed() {
    // Test with empty passphrase
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();
    assert_eq!(seed.len(), 64);

    // Test with non-empty passphrase
    let seed_with_passphrase = mnemonic_to_seed(mnemonic, "test").unwrap();
    assert_eq!(seed_with_passphrase.len(), 64);
    assert_ne!(seed, seed_with_passphrase); // Different passphrases should produce different seeds
}

#[test]
fn test_derive_key_from_path() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let seed = mnemonic_to_seed(mnemonic, "").unwrap();

    // Test standard BIP44 path
    let path = "m/44'/0'/0'/0/0";
    let key = derive_key_from_path(&seed, path).unwrap();
    assert_eq!(key.len(), 32); // Private key should be 32 bytes

    // Test different path
    let path2 = "m/44'/0'/0'/0/1";
    let key2 = derive_key_from_path(&seed, path2).unwrap();
    assert_eq!(key2.len(), 32);
    assert_ne!(key, key2); // Different paths should produce different keys

    // Test invalid path
    assert!(derive_key_from_path(&seed, "invalid/path").is_err());
}
