mod derive;
mod eth;
mod path;
mod solana;
mod sui;

pub use derive::*;
pub use eth::*;
pub use path::*;
pub use solana::*;
pub use sui::*;
use typed_builder::TypedBuilder;

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
pub struct BlockChainKeyPair {
    pub private_key: Vec<u8>,
    pub public_address: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
pub struct BlockChainPublicKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}
