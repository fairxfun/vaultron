use crate::EnclaveAttestationError;
use aws_nitro_enclaves_cose::crypto::Openssl;
use aws_nitro_enclaves_cose::CoseSign1;
use aws_nitro_enclaves_nsm_api::api::AttestationDoc;
use enclave_protos::vaultron::attestation::v1::AttestationDocument as AttestationDocProto;
use openssl::{
    stack::Stack,
    x509::{
        store::{X509Store, X509StoreBuilder},
        X509StoreContext, X509,
    },
};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_ATTESTATION_AGE_SECONDS: u64 = 300;
const ROOT_CERT_PATH: &str = "/fairx/certs/aws-nitro-enclaves-root.pem";

pub struct AttestationParser {
    max_attestation_age_seconds: u64,
    store: X509Store,
}

impl AttestationParser {
    pub async fn new(
        root_cert_path: Option<String>,
        max_attestation_age_seconds: Option<u64>,
    ) -> Result<Self, EnclaveAttestationError> {
        let max_attestation_age_seconds = max_attestation_age_seconds.unwrap_or(MAX_ATTESTATION_AGE_SECONDS);
        let cert_path = root_cert_path.unwrap_or(ROOT_CERT_PATH.to_string());
        let root_cert_pem = tokio::fs::read(&cert_path).await?;
        let root_cert = X509::from_pem(&root_cert_pem)?;
        let mut store_builder = X509StoreBuilder::new()?;
        store_builder.add_cert(root_cert.clone())?;
        let store = store_builder.build();
        Ok(Self {
            store,
            max_attestation_age_seconds,
        })
    }

    pub fn verify_and_parse(
        &self,
        attestation_bytes: &[u8],
        expected_pcr0: &[u8],
    ) -> Result<AttestationDocProto, EnclaveAttestationError> {
        let cose_sign1 = CoseSign1::from_bytes(attestation_bytes)?;

        let payload = cose_sign1.get_payload::<Openssl>(None)?;
        let doc = serde_cbor::from_slice::<AttestationDoc>(&payload)?;

        // Verify the certificate chain
        let cert_chain = doc.certificate.as_slice();
        let ca_bundle = doc.cabundle.iter().map(|ca| ca.as_slice()).collect::<Vec<_>>();
        let valid = self.verify_cert_chain(cert_chain, &ca_bundle)?;
        if !valid {
            return Err(EnclaveAttestationError::InvalidCertificateChain);
        }

        let leaf_cert = X509::from_der(&doc.certificate)?;
        let leaf_pubkey = leaf_cert.public_key()?;

        // Verify the COSE Sign1 signature using the leaf certificate's public key
        let result = cose_sign1.verify_signature::<Openssl>(&leaf_pubkey)?;
        if !result {
            return Err(EnclaveAttestationError::InvalidSignature);
        }

        // Verify timestamp is not too old
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| EnclaveAttestationError::InvalidSignature)?
            .as_secs();
        let doc_time = doc.timestamp / 1000;
        if current_time - doc_time > self.max_attestation_age_seconds {
            return Err(EnclaveAttestationError::InvalidSignature);
        }

        // Verify PCR0 matches expected value
        let pcr0 = doc.pcrs.get(&0).ok_or(EnclaveAttestationError::Pcr0Mismatch)?;
        if pcr0 != expected_pcr0 {
            return Err(EnclaveAttestationError::Pcr0Mismatch);
        }

        let doc_proto = AttestationDocProto::try_from(doc)?;
        Ok(doc_proto)
    }

    fn verify_cert_chain(&self, end_cert_der: &[u8], ca_bundle_der: &[&[u8]]) -> Result<bool, EnclaveAttestationError> {
        let end_cert = X509::from_der(end_cert_der)?;
        let mut intermediates = Stack::new()?;
        for ca_der in ca_bundle_der {
            let cert = X509::from_der(ca_der)?;
            intermediates.push(cert)?;
        }
        let mut context = X509StoreContext::new()?;
        let valid = context.init(&self.store, &end_cert, intermediates.as_ref(), |ctx| ctx.verify_cert())?;
        Ok(valid)
    }
}
