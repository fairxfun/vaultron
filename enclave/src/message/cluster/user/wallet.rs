use enclave_crypto::{
    derive_eth_keypair, derive_solana_keypair, derive_sui_keypair, generate_mnemonic, mnemonic_to_seed,
    BlockChainKeyPair, EnclaveCryptoError,
};

#[derive(Debug, PartialEq, Eq)]
pub struct MultiChainWallet {
    pub mnemonic: String,
    pub eth_keypair: BlockChainKeyPair,
    pub solana_keypair: BlockChainKeyPair,
    pub sui_keypair: BlockChainKeyPair,
}

pub fn generate_multi_chain_wallet() -> Result<MultiChainWallet, EnclaveCryptoError> {
    let mnemonic = generate_mnemonic(24)?;
    let seed = mnemonic_to_seed(&mnemonic, "")?;
    let eth_keypair = derive_eth_keypair(&seed, None)?;
    let solana_keypair = derive_solana_keypair(&seed, None)?;
    let sui_keypair = derive_sui_keypair(&seed, None)?;
    Ok(MultiChainWallet {
        mnemonic,
        eth_keypair,
        solana_keypair,
        sui_keypair,
    })
}

// pub fn recover_multi_chain_wallet(mnemonic: &str, index: Option<u32>) -> Result<MultiChainWallet, EnclaveCryptoError> {
//     let seed = mnemonic_to_seed(mnemonic, "")?;
//     let eth_keypair = derive_eth_keypair(&seed, index)?;
//     let solana_keypair = derive_solana_keypair(&seed, index)?;
//     let sui_keypair = derive_sui_keypair(&seed, index)?;
//     Ok(MultiChainWallet {
//         mnemonic: mnemonic.to_string(),
//         eth_keypair,
//         solana_keypair,
//         sui_keypair,
//     })
// }
