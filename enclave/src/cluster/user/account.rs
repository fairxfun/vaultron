use crate::common::EnclaveError;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

macro_rules! impl_serialization {
    ($type:ty) => {
        impl $type {
            pub fn to_bytes(&self) -> Result<Vec<u8>, EnclaveError> {
                postcard::to_allocvec(self).map_err(|err| err.into())
            }
            // pub fn from_bytes(bytes: &[u8]) -> Result<Self, EnclaveError> {
            //     postcard::from_bytes(bytes).map_err(|err| err.into())
            // }
        }
    };
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
pub struct UserAccountMnemonicPair {
    pub user_id: String,
    pub signature_type: i32,
    pub user_public_key: Vec<u8>,
    pub mnemonic: String,
}

impl_serialization!(UserAccountMnemonicPair);
