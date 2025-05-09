use crate::encrypt::random_len;
use enclave_crypto::{random_bytes_by_rng, AesGcmData, AesGcmKey, AES_GCM_NONCE_LENGTH, ENCRYPTION_KEY_LENGTH};

#[tokio::test]
pub async fn test_aes_encrypt_decrypt() {
    let seed = random_bytes_by_rng(ENCRYPTION_KEY_LENGTH);
    let aes_key = AesGcmKey::new(&seed).unwrap();
    let nonce = random_bytes_by_rng(AES_GCM_NONCE_LENGTH);
    let data_len = random_len(1, 10240);
    let data = random_bytes_by_rng(data_len);
    let encrypted_data = aes_key.encrypt(&nonce, &data).unwrap();
    let aes_gcm_data = AesGcmData::deserialize(&encrypted_data).unwrap();
    assert_eq!(aes_gcm_data.nonce, nonce);
    let decrypted_data = aes_key.decrypt(&encrypted_data).unwrap();
    assert_eq!(data, decrypted_data);
}
