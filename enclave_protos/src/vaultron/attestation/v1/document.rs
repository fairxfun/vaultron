pub use crate::gen::vaultron::attestation::v1::*;

use aws_nitro_enclaves_nsm_api::api::{AttestationDoc, Digest as DigestApi};
use prost::Message;

impl From<DigestApi> for Digest {
    fn from(value: DigestApi) -> Self {
        match value {
            DigestApi::SHA256 => Digest::Sha256,
            DigestApi::SHA384 => Digest::Sha384,
            DigestApi::SHA512 => Digest::Sha512,
        }
    }
}

impl From<Digest> for DigestApi {
    fn from(value: Digest) -> Self {
        match value {
            Digest::Unspecified => DigestApi::SHA384,
            Digest::Sha256 => DigestApi::SHA256,
            Digest::Sha384 => DigestApi::SHA384,
            Digest::Sha512 => DigestApi::SHA512,
        }
    }
}

impl TryFrom<AttestationDoc> for AttestationDocument {
    type Error = anyhow::Error;

    fn try_from(value: AttestationDoc) -> Result<Self, Self::Error> {
        let user_data_buf = value.user_data.ok_or(anyhow::anyhow!("User data is not present"))?;
        let user_data = AttestationDocumentUserData::decode(&mut user_data_buf.as_slice())
            .map_err(|_| anyhow::anyhow!("Failed to decode user data"))?;
        let digest_proto: Digest = value.digest.into();
        Ok(AttestationDocument {
            module_id: value.module_id,
            digest: digest_proto as i32,
            timestamp: value.timestamp,
            pcrs: value
                .pcrs
                .into_iter()
                .map(|(k, v)| (k as u32, v.as_slice().to_vec()))
                .collect(),
            certificate: value.certificate.as_slice().to_vec(),
            cabundle: value.cabundle.into_iter().map(|v| v.as_slice().to_vec()).collect(),
            public_key: value.public_key.unwrap_or_default().as_slice().to_vec(),
            nonce: value.nonce.unwrap_or_default().as_slice().to_vec(),
            user_data: Some(user_data),
        })
    }
}

impl From<AttestationDocument> for AttestationDoc {
    fn from(value: AttestationDocument) -> Self {
        let user_data = value.user_data.map(|user_data| user_data.encode_to_vec());
        let nonce = if value.nonce.is_empty() {
            None
        } else {
            Some(value.nonce.as_slice().to_vec())
        };
        let public_key = if value.public_key.is_empty() {
            None
        } else {
            Some(value.public_key.as_slice().to_vec())
        };
        let pcrs = value
            .pcrs
            .into_iter()
            .map(|(k, v)| (k as usize, v.as_slice().to_vec()))
            .collect();
        let digest = Digest::try_from(value.digest).unwrap_or(Digest::Unspecified);
        AttestationDoc::new(
            value.module_id,
            digest.into(),
            value.timestamp,
            pcrs,
            value.certificate,
            value.cabundle,
            public_key,
            nonce,
            user_data,
        )
    }
}

impl AttestationDocumentUserData {
    pub fn encode_message<T: Message, P: Message>(request: &T, response: &P) -> Vec<u8> {
        let request_buf = request.encode_to_vec();
        let response_buf = response.encode_to_vec();
        let data = AttestationDocumentUserData::builder()
            .request(request_buf)
            .response(response_buf)
            .build();
        data.encode_to_vec()
    }

    pub fn decode_to_message<T: Message + Default, P: Message + Default>(&self) -> Result<(T, P), prost::DecodeError> {
        let request = T::decode(&mut self.request.as_slice())?;
        let response = P::decode(&mut self.response.as_slice())?;
        Ok((request, response))
    }
}

impl AttestationDocument {
    pub fn get_pcr0(&self) -> Result<&[u8], prost::DecodeError> {
        let pcr0 = self.pcrs.get(&0).ok_or(prost::DecodeError::new("PCR0 not found"))?;
        Ok(pcr0)
    }

    pub fn decode_user_data<T: Message + Default, P: Message + Default>(&self) -> Result<(T, P), prost::DecodeError> {
        let user_data = self
            .user_data
            .as_ref()
            .ok_or(prost::DecodeError::new("User data not found"))?;
        user_data.decode_to_message()
    }
}
