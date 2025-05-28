use super::inner::ClusterMessageHandlerInner;
use crate::cluster::{generate_multi_chain_wallet, MultiChainWallet, UserAccountMnemonicPair};
use crate::common::EnclaveError;
use enclave_crypto::EciesKeyPair;
use enclave_protos::vaultron::enclave::cluster::v1::{
    CreateEnclaveVaultronRequest, CreateEnclaveVaultronResponse, VaultronCreationAttributes, VaultronUserSignatureType,
};
use enclave_utils::address::ethers_address_from_bytes;
use ethers_core::types::Signature;
use log::info;
use prost::Message;
use std::convert::TryFrom;

impl ClusterMessageHandlerInner {
    pub(crate) async fn handle_create_user_vaultron_request(
        &self,
        request: &CreateEnclaveVaultronRequest,
    ) -> Result<CreateEnclaveVaultronResponse, EnclaveError> {
        info!("Received create enclave wallet request");
        let attributes: &VaultronCreationAttributes =
            request.attributes.as_ref().ok_or(EnclaveError::InvalidRequestError)?;
        self.verify(attributes, &request.signature)?;
        let wallet = generate_multi_chain_wallet()?;
        let response = self.build_vaultron_creation_response(attributes, &wallet).await?;
        Ok(response)
    }

    fn verify(&self, attributes: &VaultronCreationAttributes, signature: &[u8]) -> Result<(), EnclaveError> {
        self.verify_vaultron_attributes(attributes)?;
        self.verify_user_signature(attributes, signature)?;
        Ok(())
    }

    fn verify_vaultron_attributes(&self, attributes: &VaultronCreationAttributes) -> Result<(), EnclaveError> {
        match attributes.cluster.as_ref() {
            Some(cluster) => {
                if !self.is_current_cluster(&cluster.enclave_pcr0) {
                    return Err(EnclaveError::InvalidParameterError);
                }
                Ok(())
            }
            None => Err(EnclaveError::InvalidRequestError),
        }
    }

    fn verify_user_signature(
        &self,
        attributes: &VaultronCreationAttributes,
        signature: &[u8],
    ) -> Result<(), EnclaveError> {
        match attributes.signature_type() {
            VaultronUserSignatureType::WalletEth => self.verify_evm_signature(attributes, signature),
            // TODO: add other signature types
            _ => Err(EnclaveError::InvalidRequestError),
        }
    }

    fn verify_evm_signature(
        &self,
        attributes: &VaultronCreationAttributes,
        signature: &[u8],
    ) -> Result<(), EnclaveError> {
        let user_account =
            ethers_address_from_bytes(&attributes.public_key).map_err(|_| EnclaveError::InvalidAccountError)?;
        let signature = Signature::try_from(signature).map_err(|_| EnclaveError::InvalidSignatureError)?;
        let request_message = attributes.encode_to_vec();
        signature
            .verify(request_message, user_account)
            .map_err(|_| EnclaveError::InvalidSignatureError)?;
        Ok(())
    }

    async fn build_vaultron_creation_response(
        &self,
        attributes: &VaultronCreationAttributes,
        wallet: &MultiChainWallet,
    ) -> Result<CreateEnclaveVaultronResponse, EnclaveError> {
        let account_pair = UserAccountMnemonicPair::builder()
            .user_id(attributes.user_id.clone())
            .user_public_key(attributes.public_key.clone())
            .signature_type(attributes.signature_type)
            .mnemonic(wallet.mnemonic.clone())
            .build();
        let kms_mnemonic_bytes = account_pair.to_bytes()?;
        let encrypted_account_pair = self.cluster_key.encrypt(&kms_mnemonic_bytes)?;

        let ecies_key_pair = EciesKeyPair::from_public_key(attributes.recovery_public_key.clone())?;
        let encrypted_seed = ecies_key_pair.encrypt_by_public_key(wallet.mnemonic.as_bytes())?;

        let response = CreateEnclaveVaultronResponse::builder()
            .encrypted_attributes(encrypted_account_pair)
            .encrypted_seed(encrypted_seed)
            .eth_public_key(wallet.eth_keypair.public_address.clone())
            .solana_public_key(wallet.solana_keypair.public_address.clone())
            .sui_public_key(wallet.sui_keypair.public_address.clone())
            .build();
        Ok(response)
    }

    fn is_current_cluster(&self, pcr0: &[u8]) -> bool {
        self.context.settings.pcr0 == pcr0.to_vec()
    }
}
