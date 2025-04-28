use vaultron_enclave::start_vaultron_enclave;
#[tokio::main]
async fn main() {
    let result = start_vaultron_enclave().await;
    println!("Server exit: {:?}", result);
}
