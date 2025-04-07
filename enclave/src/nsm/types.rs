use aws_nitro_enclaves_nsm_api::api::Request;
use typed_builder::TypedBuilder;

pub const ATTESTATION_NONCE_SIZE: usize = 32;

#[derive(Debug, TypedBuilder)]
pub struct AttestationRequest {
    pub user_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl From<AttestationRequest> for Request {
    fn from(value: AttestationRequest) -> Self {
        Request::Attestation {
            user_data: Some(value.user_data.into()),
            nonce: Some(value.nonce.into()),
            public_key: Some(value.public_key.into()),
        }
    }
}
