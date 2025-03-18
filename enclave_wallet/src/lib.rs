mod derive;
mod error;
mod eth;
mod path;
mod solana;
mod sui;

pub use error::*;

use crate::derive::{generate_mnemonic, mnemonic_to_seed};
use crate::eth::derive_eth_keypair;
use crate::solana::derive_solana_keypair;
use crate::sui::derive_sui_keypair;
use path::{get_eth_derivation_path, get_solana_derivation_path, get_sui_derivation_path};

#[derive(Debug, PartialEq, Eq)]
pub struct MultiChainWallet {
    pub mnemonic: String,
    pub eth_keypair: EnclaveKeyPair,
    pub solana_keypair: EnclaveKeyPair,
    pub sui_keypair: EnclaveKeyPair,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnclaveKeyPair {
    pub private_key: Vec<u8>,
    pub public_address: Vec<u8>,
}

pub fn generate_multi_chain_wallet() -> Result<MultiChainWallet, EnclaveWalletError> {
    let mnemonic = generate_mnemonic(24)?;
    let seed = mnemonic_to_seed(&mnemonic, "")?;
    let eth_keypair = derive_eth_keypair(&seed, &{ get_eth_derivation_path }(None))?;
    let solana_keypair = derive_solana_keypair(&seed, &get_solana_derivation_path(None))?;
    let sui_keypair = derive_sui_keypair(&seed, &get_sui_derivation_path(None))?;
    Ok(MultiChainWallet {
        mnemonic,
        eth_keypair,
        solana_keypair,
        sui_keypair,
    })
}

pub fn recover_multi_chain_wallet(mnemonic: &str, index: Option<u32>) -> Result<MultiChainWallet, EnclaveWalletError> {
    let seed = mnemonic_to_seed(mnemonic, "")?;
    let eth_keypair = derive_eth_keypair(&seed, &get_eth_derivation_path(index))?;
    let solana_keypair = derive_solana_keypair(&seed, &get_solana_derivation_path(index))?;
    let sui_keypair = derive_sui_keypair(&seed, &get_sui_derivation_path(index))?;
    Ok(MultiChainWallet {
        mnemonic: mnemonic.to_string(),
        eth_keypair,
        solana_keypair,
        sui_keypair,
    })
}
