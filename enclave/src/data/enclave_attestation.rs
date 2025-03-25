use crate::common::EnclaveError;
use ciborium::de::from_reader;
use ciborium::value::Value;
use enclave_utils::hex::encode_hex;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize)]
pub struct AttestationDocument {
    #[serde(rename = "module_id")]
    pub module_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "digest")]
    pub digest: String,
    #[serde(rename = "pcrs")]
    pub pcrs: BTreeMap<u8, Vec<u8>>,
    #[serde(rename = "certificate")]
    pub certificate: Vec<u8>,
    #[serde(rename = "cabundle")]
    pub cabundle: Vec<Vec<u8>>,
    #[serde(rename = "public_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Vec<u8>>,
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Vec<u8>>,
    #[serde(rename = "nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Vec<u8>>,
}

pub fn get_pcr_from_attestation_doc(attestation_doc: &[u8]) -> Result<Vec<String>, EnclaveError> {
    let doc = decode_attestation_doc(attestation_doc)?;
    let pcrs = doc.pcrs.values().map(encode_hex).collect::<Vec<String>>();
    if pcrs.is_empty() {
        return Err(EnclaveError::InvalidAttestationDocumentError);
    }
    Ok(pcrs)
}

pub fn decode_attestation_doc(attestation_doc: &[u8]) -> Result<AttestationDocument, EnclaveError> {
    // First decode as a COSE_Sign1 structure (CBOR array)
    // ref: https://github.com/aws/aws-nitro-enclaves-nsm-api/blob/main/docs/attestation_process.md#22-attestation-document-specification
    let cose_sign1: Vec<Value> = from_reader(attestation_doc).map_err(|e| {
        error!("Failed to decode cose_sign1: {:?}", e);
        EnclaveError::InvalidAttestationDocumentError
    })?;

    // The COSE_Sign1 structure is a CBOR array with 4 elements:
    // 1. protected header (bstr)
    // 2. unprotected header (map)
    // 3. payload (bstr) - this is our attestation document
    // 4. signature (bstr)

    // Get the payload (index 2) which contains our attestation document
    let payload = cose_sign1
        .get(2)
        .and_then(|v| v.as_bytes())
        .ok_or(EnclaveError::InvalidAttestationDocumentError)?;

    // Now decode the payload as our attestation document
    let mut cursor = Cursor::new(&payload);
    let doc: AttestationDocument = from_reader(&mut cursor).map_err(|e| {
        error!("Failed to decode attestation document: {:?}", e);
        EnclaveError::InvalidAttestationDocumentError
    })?;
    Ok(doc)
}
