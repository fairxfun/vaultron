use super::MessageHandler;

use crate::{
    common::EnclaveError, data::KmsAccountEthPair, data::KmsAccountMnemonicPair, data::KmsAccountSolanaPair,
    data::KmsAccountSuiPair,
};
use enclave_kmstool::KmsEncryptRequest;
use enclave_protos::enclave::v1::{CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, SignatureType, StatusCode};
use enclave_utils::address::ethers_address_from_bytes;
use enclave_wallet::generate_multi_chain_wallet;
use ethers_core::types::Signature;
use log::info;
use std::convert::TryFrom;

impl MessageHandler {
    pub(crate) async fn handle_create_enclave_wallet_request(
        &self,
        request: CreateEnclaveWalletRequest,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveError> {
        info!("Received create enclave wallet request: {:?}", request);
        self.verify_signature(&request)?;
        let wallet: enclave_wallet::MultiChainWallet = generate_multi_chain_wallet()?;
        let response = self.kms_encrypt_wallet_data(&request, &wallet)?;
        Ok(response)
    }

    fn verify_signature(&self, request: &CreateEnclaveWalletRequest) -> Result<(), EnclaveError> {
        match request.signature_type() {
            SignatureType::WalletEth => self.verify_evm_signature(request),
            // TODO: add other signature types
            _ => Err(EnclaveError::InvalidRequestError),
        }
    }

    fn verify_evm_signature(&self, request: &CreateEnclaveWalletRequest) -> Result<(), EnclaveError> {
        let user_account =
            ethers_address_from_bytes(&request.user_public_key).map_err(|_| EnclaveError::InvalidAccountError)?;
        let signature =
            Signature::try_from(request.signature.as_slice()).map_err(|_| EnclaveError::InvalidSignatureError)?;
        let request_message_hash = ethers_core::utils::hash_message(request.message.as_slice());
        let recovered_account = signature
            .recover(request_message_hash)
            .map_err(|_| EnclaveError::InvalidSignatureError)?;
        if recovered_account != user_account {
            return Err(EnclaveError::InvalidAccountError);
        }
        Ok(())
    }

    fn kms_encrypt_wallet_data(
        &self,
        request: &CreateEnclaveWalletRequest,
        wallet: &enclave_wallet::MultiChainWallet,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveError> {
        let kms_mnemonic = KmsAccountMnemonicPair::builder()
            .user_id(request.user_id.clone())
            .user_public_key(request.user_public_key.clone())
            .mnemonic(wallet.mnemonic.clone())
            .build();
        let kms_mnemonic_bytes = kms_mnemonic.to_bytes()?;
        let ksm_response = self
            .context
            .kms_client
            .encrypt(KmsEncryptRequest::builder().plaintext(kms_mnemonic_bytes).build())?;
        let encrypted_kms_mnemonic = ksm_response.ciphertext;

        let kms_evm = KmsAccountEthPair::builder()
            .user_id(request.user_id.clone())
            .user_public_key(request.user_public_key.clone())
            .eth_private_key(wallet.eth_keypair.private_key.clone())
            .build();
        let kms_evm_bytes = kms_evm.to_bytes()?;
        let ksm_response = self
            .context
            .kms_client
            .encrypt(KmsEncryptRequest::builder().plaintext(kms_evm_bytes).build())?;
        let encrypted_kms_evm = ksm_response.ciphertext;

        let kms_solana = KmsAccountSolanaPair::builder()
            .user_id(request.user_id.clone())
            .user_public_key(request.user_public_key.clone())
            .solana_private_key(wallet.solana_keypair.private_key.clone())
            .build();
        let kms_solana_bytes = kms_solana.to_bytes()?;
        let ksm_response = self
            .context
            .kms_client
            .encrypt(KmsEncryptRequest::builder().plaintext(kms_solana_bytes).build())?;
        let encrypted_kms_solana = ksm_response.ciphertext;

        let kms_sui = KmsAccountSuiPair::builder()
            .user_id(request.user_id.clone())
            .user_public_key(request.user_public_key.clone())
            .sui_private_key(wallet.sui_keypair.private_key.clone())
            .build();
        let kms_sui_bytes = kms_sui.to_bytes()?;
        let ksm_response = self
            .context
            .kms_client
            .encrypt(KmsEncryptRequest::builder().plaintext(kms_sui_bytes).build())?;
        let encrypted_kms_sui = ksm_response.ciphertext;

        let response = CreateEnclaveWalletResponse::builder()
            .code(StatusCode::success())
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
}
