use crate::common::EnclaveError;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

macro_rules! impl_serialization {
    ($type:ty) => {
        impl $type {
            pub fn to_bytes(&self) -> Result<Vec<u8>, EnclaveError> {
                postcard::to_allocvec(self).map_err(|err| err.into())
            }
            pub fn from_bytes(bytes: &[u8]) -> Result<Self, EnclaveError> {
                postcard::from_bytes(bytes).map_err(|err| err.into())
            }
        }
    };
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
pub struct KmsAccountMnemonicPair {
    pub user_id: Vec<u8>,
    pub user_public_key: Vec<u8>,
    pub mnemonic: String,
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
pub struct KmsAccountEthPair {
    pub user_id: Vec<u8>,
    pub user_public_key: Vec<u8>,
    pub eth_private_key: Vec<u8>,
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
pub struct KmsAccountSolanaPair {
    pub user_id: Vec<u8>,
    pub user_public_key: Vec<u8>,
    pub solana_private_key: Vec<u8>,
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
pub struct KmsAccountSuiPair {
    pub user_id: Vec<u8>,
    pub user_public_key: Vec<u8>,
    pub sui_private_key: Vec<u8>,
}

impl_serialization!(KmsAccountMnemonicPair);
impl_serialization!(KmsAccountEthPair);
impl_serialization!(KmsAccountSolanaPair);
impl_serialization!(KmsAccountSuiPair);
