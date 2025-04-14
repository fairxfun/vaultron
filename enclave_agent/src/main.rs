use enclave_agent::{start_agent, EnclaveAgentCreateOptions};

#[tokio::main]
async fn main() {
    let options = EnclaveAgentCreateOptions::default();
    let result = start_agent(options).await;
    match result {
        Ok(_) => println!("Enclave agent exited without error"),
        Err(e) => println!("Enclave agent exited with error: {}", e),
    }
}
