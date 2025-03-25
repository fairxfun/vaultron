use enclave_kmstool::EnclaveKmstoolError;
use enclave_kmstool::KmsToolTrait;
use enclave_kmstool::KmstoolDecryptParams;
use enclave_kmstool::KmstoolDecryptResult;
use enclave_kmstool::KmstoolEncryptParams;
use enclave_kmstool::KmstoolEncryptResult;
use enclave_kmstool::KmstoolGetAttestationDocumentResult;
use enclave_kmstool::KmstoolGetKeyPolicyParams;
use enclave_kmstool::KmstoolGetKeyPolicyResult;
use enclave_kmstool::KmstoolInitParams;
use enclave_kmstool::KmstoolListKeyPoliciesParams;
use enclave_kmstool::KmstoolListKeyPoliciesResult;
use enclave_kmstool::KmstoolUpdateAwsCredentialsParams;
use mockall::*;

mock! {
    #[derive(Debug)]
    pub KmstoolClient {}

    impl KmsToolTrait for KmstoolClient {
        fn init(&self, request: KmstoolInitParams) -> Result<(), EnclaveKmstoolError>;
        fn update_aws_credentials(&self, request: KmstoolUpdateAwsCredentialsParams) -> Result<(), EnclaveKmstoolError>;
        fn get_attestation_document(&self) -> Result<KmstoolGetAttestationDocumentResult, EnclaveKmstoolError>;
        fn list_key_policies(&self, request: KmstoolListKeyPoliciesParams) -> Result<KmstoolListKeyPoliciesResult, EnclaveKmstoolError>;
        fn get_key_policy(&self, request: KmstoolGetKeyPolicyParams) -> Result<KmstoolGetKeyPolicyResult, EnclaveKmstoolError>;
        fn encrypt(&self, request: KmstoolEncryptParams) -> Result<KmstoolEncryptResult, EnclaveKmstoolError>;
        fn decrypt(&self, request: KmstoolDecryptParams) -> Result<KmstoolDecryptResult, EnclaveKmstoolError>;
    }
}
