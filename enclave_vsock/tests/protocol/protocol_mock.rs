use enclave_vsock::ProtocolStreamTrait;
use enclave_vsock::VsockProtocolError;
use mockall::predicate::*;
use mockall::*;

mock! {
    pub ProtocolStream {}

    impl ProtocolStreamTrait for ProtocolStream {
        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), VsockProtocolError>;
        fn write_all(&mut self, buf: &[u8]) -> Result<(), VsockProtocolError>;
        fn flush(&mut self) -> Result<(), VsockProtocolError>;
    }
}
