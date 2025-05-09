use super::inner::ClusterMessageHandlerInner;
use crate::cluster::{
    generate_multi_chain_wallet, MultiChainWallet, UserAccountEthPair, UserAccountMnemonicPair, UserAccountSolanaPair,
    UserAccountSuiPair,
};
use crate::common::EnclaveError;
use enclave_protos::vaultron::enclave::cluster::v1::{
    CreateUserWalletParams, CreateUserWalletRequest, CreateUserWalletResponse, SignatureType,
};
use enclave_utils::address::ethers_address_from_bytes;
use ethers_core::types::Signature;
use log::info;
use prost::Message;
use std::convert::TryFrom;

impl ClusterMessageHandlerInner {
    pub(crate) async fn handle_create_user_wallet_request(
        &self,
        request: &CreateUserWalletRequest,
    ) -> Result<CreateUserWalletResponse, EnclaveError> {
        info!("Received create enclave wallet request");
        let params: &CreateUserWalletParams = request.params.as_ref().ok_or(EnclaveError::InvalidRequestError)?;
        self.verify(params, &request.signature)?;
        let wallet = generate_multi_chain_wallet()?;
        let response = self.encrypt_wallet_data(params, &wallet).await?;
        Ok(response)
    }

    fn verify(&self, params: &CreateUserWalletParams, signature: &[u8]) -> Result<(), EnclaveError> {
        self.verify_cluster_attributes(params)?;
        self.verify_signature(params, signature)?;
        Ok(())
    }

    fn verify_cluster_attributes(&self, params: &CreateUserWalletParams) -> Result<(), EnclaveError> {
        match params.attributes.as_ref() {
            Some(data) => {
                if !self.is_current_cluster(&data.enclave_pcr0) {
                    return Err(EnclaveError::InvalidParameterError);
                }
                Ok(())
            }
            None => Err(EnclaveError::InvalidRequestError),
        }
    }

    fn verify_signature(&self, params: &CreateUserWalletParams, signature: &[u8]) -> Result<(), EnclaveError> {
        match params.signature_type() {
            SignatureType::WalletEth => self.verify_evm_signature(params, signature),
            // TODO: add other signature types
            _ => Err(EnclaveError::InvalidRequestError),
        }
    }

    fn verify_evm_signature(&self, params: &CreateUserWalletParams, signature: &[u8]) -> Result<(), EnclaveError> {
        let user_account =
            ethers_address_from_bytes(&params.user_public_key).map_err(|_| EnclaveError::InvalidAccountError)?;
        let signature = Signature::try_from(signature).map_err(|_| EnclaveError::InvalidSignatureError)?;
        let request_message = params.encode_to_vec();
        signature
            .verify(request_message, user_account)
            .map_err(|_| EnclaveError::InvalidSignatureError)?;
        Ok(())
    }

    async fn encrypt_wallet_data(
        &self,
        params: &CreateUserWalletParams,
        wallet: &MultiChainWallet,
    ) -> Result<CreateUserWalletResponse, EnclaveError> {
        let kms_mnemonic = UserAccountMnemonicPair::builder()
            .user_id(params.user_id.clone())
            .user_public_key(params.user_public_key.clone())
            .mnemonic(wallet.mnemonic.clone())
            .build();
        let kms_mnemonic_bytes = kms_mnemonic.to_bytes()?;
        let encrypted_kms_mnemonic = self.cluster_key.encrypt(&kms_mnemonic_bytes)?;

        let kms_evm = UserAccountEthPair::builder()
            .user_id(params.user_id.clone())
            .user_public_key(params.user_public_key.clone())
            .eth_private_key(wallet.eth_keypair.private_key.clone())
            .build();
        let kms_evm_bytes = kms_evm.to_bytes()?;
        let encrypted_kms_evm = self.cluster_key.encrypt(&kms_evm_bytes)?;

        let kms_solana = UserAccountSolanaPair::builder()
            .user_id(params.user_id.clone())
            .user_public_key(params.user_public_key.clone())
            .solana_private_key(wallet.solana_keypair.private_key.clone())
            .build();
        let kms_solana_bytes = kms_solana.to_bytes()?;
        let encrypted_kms_solana = self.cluster_key.encrypt(&kms_solana_bytes)?;

        let kms_sui = UserAccountSuiPair::builder()
            .user_id(params.user_id.clone())
            .user_public_key(params.user_public_key.clone())
            .sui_private_key(wallet.sui_keypair.private_key.clone())
            .build();
        let kms_sui_bytes = kms_sui.to_bytes()?;
        let encrypted_kms_sui = self.cluster_key.encrypt(&kms_sui_bytes)?;

        let response = CreateUserWalletResponse::builder()
            .wallet_encrypted_data(encrypted_kms_mnemonic)
            .eth_public_key(wallet.eth_keypair.public_address.clone())
            .eth_encrypted_data(encrypted_kms_evm)
            .solana_public_key(wallet.solana_keypair.public_address.clone())
            .solana_encrypted_data(encrypted_kms_solana)
            .sui_public_key(wallet.sui_keypair.public_address.clone())
            .sui_encrypted_data(encrypted_kms_sui)
            .build();
        Ok(response)
    }

    fn is_current_cluster(&self, pcr0: &[u8]) -> bool {
        self.context.settings.pcr0 == pcr0.to_vec()
    }
}
