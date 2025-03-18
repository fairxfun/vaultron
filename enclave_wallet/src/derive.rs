use crate::EnclaveWalletError;
use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use std::str::FromStr;

pub fn generate_mnemonic(word_count: usize) -> Result<String, EnclaveWalletError> {
    let mnemonic = Mnemonic::generate(word_count)?;
    Ok(mnemonic.to_string())
}

pub fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Result<Vec<u8>, EnclaveWalletError> {
    let salt = format!("mnemonic{}", passphrase);
    let mut seed = vec![0u8; 64];
    match pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut seed) {
        Ok(_) => Ok(seed),
        Err(_) => Err(EnclaveWalletError::WalletGenerationError),
    }
}

pub fn derive_key_from_path(seed: &[u8], path: &str) -> Result<Vec<u8>, EnclaveWalletError> {
    let xprv = XPrv::new(seed).map_err(|_| EnclaveWalletError::WalletGenerationError)?;
    let derivation_path = DerivationPath::from_str(path).map_err(|_| EnclaveWalletError::WalletGenerationError)?;
    let mut current_key = xprv;
    for child_number in derivation_path.iter() {
        current_key = current_key
            .derive_child(child_number)
            .map_err(|_| EnclaveWalletError::WalletGenerationError)?;
    }
    let private_key = current_key.private_key().to_bytes().to_vec();
    Ok(private_key)
}
