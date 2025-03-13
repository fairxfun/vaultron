use bip39::Mnemonic;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_signers::Wallet;
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey};
use fastcrypto::traits::{KeyPair as FastCryptoKeyPair, ToFromBytes};
use hmac::Hmac;
use hmac::Mac;
use k256::{
    ecdsa::{SigningKey as K256SigningKey, VerifyingKey as K256VerifyingKey},
    elliptic_curve::SecretKey as K256SecretKey,
};
use log::{debug, error, info};
use pbkdf2::pbkdf2;
use sha2::Sha512;
use solana_sdk::signature::{Keypair as SolanaKeypair, Signer};

use crate::EnclaveWalletError;

const ETH_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";
const SOLANA_DERIVATION_PATH: &str = "m/44'/501'/0'/0'";
const SUI_DERIVATION_PATH: &str = "m/44'/784'/0'/0'/0'";

#[derive(Debug)]
pub struct MultiChainWallet {
    pub mnemonic: String,
    pub eth_keypair: KeyPair,
    pub solana_keypair: KeyPair,
    pub sui_keypair: KeyPair,
}

#[derive(Debug)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub fn generate_multi_chain_wallet() -> Result<MultiChainWallet, EnclaveWalletError> {
    let mnemonic = generate_mnemonic(24)?;
    let seed = mnemonic_to_seed(&mnemonic, "")?; // 空密码

    debug!("Generated mnemonic with {} words", mnemonic.split_whitespace().count());

    let eth_keypair = derive_eth_keypair(&seed, ETH_DERIVATION_PATH)?;
    let solana_keypair = derive_solana_keypair(&seed, SOLANA_DERIVATION_PATH)?;
    let sui_keypair = derive_sui_keypair(&seed, SUI_DERIVATION_PATH)?;

    info!("Successfully derived keypairs for ETH, Solana, and SUI");

    Ok(MultiChainWallet {
        mnemonic,
        eth_keypair,
        solana_keypair,
        sui_keypair,
    })
}

fn generate_mnemonic(word_count: usize) -> Result<String, EnclaveWalletError> {
    let mnemonic = Mnemonic::generate(word_count)?;
    Ok(mnemonic.to_string())
}

fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Result<Vec<u8>, EnclaveWalletError> {
    let salt = format!("mnemonic{}", passphrase);
    let mut seed = vec![0u8; 64];

    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut seed)
        .map_err(|_| EnclaveWalletError::WalletGenerationError)?;

    Ok(seed)
}

fn derive_key_from_path(seed: &[u8], path: &str) -> Result<Vec<u8>, EnclaveWalletError> {
    let path_components: Vec<&str> = path.split('/').collect();
    if path_components.is_empty() || path_components[0] != "m" {
        return Err(EnclaveWalletError::WalletGenerationError);
    }

    let mut hmac_key = b"Bitcoin seed".to_vec();
    let mut chain_code = [0u8; 32];
    let mut private_key = [0u8; 32];

    let mut hmac = Hmac::<Sha512>::new_from_slice(&hmac_key).map_err(|_| EnclaveWalletError::WalletGenerationError)?;
    hmac.update(seed);
    let result = hmac.finalize().into_bytes();

    private_key.copy_from_slice(&result[0..32]);
    chain_code.copy_from_slice(&result[32..64]);

    for component in path_components.iter().skip(1) {
        let hardened = component.ends_with('\'');
        let index_str = if hardened {
            &component[0..component.len() - 1]
        } else {
            component
        };

        let mut index = index_str
            .parse::<u32>()
            .map_err(|_| EnclaveWalletError::WalletGenerationError)?;
        if hardened {
            index += 0x80000000;
        }

        hmac_key = chain_code.to_vec();
        let mut data = Vec::with_capacity(37);

        if hardened {
            data.push(0);
            data.extend_from_slice(&private_key);
        } else {
            let secp_private_key =
                K256SecretKey::from_slice(&private_key).map_err(|_| EnclaveWalletError::WalletGenerationError)?;
            let signing_key = K256SigningKey::from(&secp_private_key);
            let verifying_key = K256VerifyingKey::from(&signing_key);
            let public_key = verifying_key.to_encoded_point(false).as_bytes().to_vec();

            data.extend_from_slice(&public_key);
        }

        data.extend_from_slice(&index.to_be_bytes());

        let mut hmac =
            Hmac::<Sha512>::new_from_slice(&hmac_key).map_err(|_| EnclaveWalletError::WalletGenerationError)?;
        hmac.update(&data);
        let result = hmac.finalize().into_bytes();

        let mut child_private_key = [0u8; 32];
        child_private_key.copy_from_slice(&result[0..32]);

        let mut carry = 0u16;
        for i in (0..32).rev() {
            let sum = carry + child_private_key[i] as u16 + private_key[i] as u16;
            child_private_key[i] = (sum & 0xFF) as u8;
            carry = sum >> 8;
        }

        private_key = child_private_key;
        chain_code.copy_from_slice(&result[32..64]);
    }

    Ok(private_key.to_vec())
}

fn derive_eth_keypair(seed: &[u8], path: &str) -> Result<KeyPair, EnclaveWalletError> {
    let private_key = derive_key_from_path(seed, path)?;

    let signing_key = SigningKey::from_slice(&private_key).map_err(|e| {
        error!("Invalid ETH private key: {:?}", e);
        EnclaveWalletError::WalletGenerationError
    })?;

    let wallet = Wallet::from(signing_key);
    let signer = wallet.signer();
    let verifying_key = signer.verifying_key();
    let public_key = verifying_key.to_encoded_point(false).as_bytes().to_vec();

    debug!("ETH keypair derived successfully using ethers-core");

    Ok(KeyPair {
        private_key: private_key.to_vec(),
        public_key,
    })
}

fn derive_solana_keypair(seed: &[u8], path: &str) -> Result<KeyPair, EnclaveWalletError> {
    let private_key = derive_key_from_path(seed, path)?;

    let mut keypair_seed = [0u8; 64];
    keypair_seed[..32].copy_from_slice(&private_key);

    let keypair = SolanaKeypair::from_bytes(&keypair_seed).map_err(|e| {
        error!("Invalid Solana keypair seed: {:?}", e);
        EnclaveWalletError::WalletGenerationError
    })?;

    let public_key = keypair.pubkey().to_bytes().to_vec();
    let private_key = keypair.secret().to_bytes().to_vec();

    debug!("Solana keypair derived successfully using Solana SDK");

    Ok(KeyPair {
        private_key,
        public_key,
    })
}

fn derive_sui_keypair(seed: &[u8], path: &str) -> Result<KeyPair, EnclaveWalletError> {
    let private_key = derive_key_from_path(seed, path)?;

    let sui_private_key = Ed25519PrivateKey::from_bytes(&private_key).map_err(|e| {
        error!("Invalid SUI private key: {:?}", e);
        EnclaveWalletError::WalletGenerationError
    })?;

    let sui_keypair = Ed25519KeyPair::from(sui_private_key);

    let public_key = sui_keypair.public().as_ref().to_vec();
    let private_key = sui_keypair.private().as_ref().to_vec();

    debug!("SUI keypair derived successfully using SUI fastcrypto");

    Ok(KeyPair {
        private_key,
        public_key,
    })
}
