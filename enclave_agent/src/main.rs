use enclave_agent::args::EnclaveArgs;
use enclave_agent::client::enclave_client;

fn main() {
    let args = EnclaveArgs::builder().cid(1000).port(5001).build();
    enclave_client(args).unwrap();
}
