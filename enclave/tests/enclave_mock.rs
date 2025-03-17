use enclave_kmstool::EnclaveKmstoolError;
use enclave_kmstool::KmsDecryptRequest;
use enclave_kmstool::KmsDecryptResponse;
use enclave_kmstool::KmsEncryptRequest;
use enclave_kmstool::KmsEncryptResponse;
use enclave_kmstool::KmsInitRequest;
use enclave_kmstool::KmsToolTrait;
use enclave_kmstool::KmsUpdateAwsCredentialsRequest;
use mockall::*;

mock! {
    #[derive(Debug)]
    pub KmstoolClient {}

    impl KmsToolTrait for KmstoolClient {
        fn init(&self, request: KmsInitRequest) -> Result<(), EnclaveKmstoolError>;
        fn update_aws_credentials(&self, request: KmsUpdateAwsCredentialsRequest) -> Result<(), EnclaveKmstoolError>;
        fn encrypt(&self, request: KmsEncryptRequest) -> Result<KmsEncryptResponse, EnclaveKmstoolError>;
        fn decrypt(&self, request: KmsDecryptRequest) -> Result<KmsDecryptResponse, EnclaveKmstoolError>;
    }
}
