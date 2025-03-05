use vaultron_enclave_proxy::args::EnclaveArgs;
use vaultron_enclave_proxy::client::enclave_client;

fn main() {
    let args = EnclaveArgs::builder().cid(1000).port(5001).build();
    let _ = enclave_client(args).unwrap();
}
