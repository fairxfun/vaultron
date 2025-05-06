use crate::encrypt::random_len;
use enclave_crypto::{random_bytes_by_rng, EciesKeyPair};

#[tokio::test]
pub async fn test_ecies_encrypt_decrypt() {
    let seed = random_bytes_by_rng(32);
    let key_pair1 = EciesKeyPair::from_seed(&seed).unwrap();
    let data_len = random_len(1, 10240);
    let data = random_bytes_by_rng(data_len);
    let encrypted_data = key_pair1.encrypt_by_public_key(&data).unwrap();
    let decrypted_data = key_pair1.decrypt_by_private_key(&encrypted_data).unwrap();
    assert_eq!(data, decrypted_data);

    let key_pair2 = EciesKeyPair::from_public_key(key_pair1.public_key().to_vec()).unwrap();
    let encrypted_data = key_pair2.encrypt_by_public_key(&data).unwrap();
    let decrypted_data = key_pair1.decrypt_by_private_key(&encrypted_data).unwrap();
    assert_eq!(data, decrypted_data);
}
