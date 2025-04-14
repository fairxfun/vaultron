use vaultron_enclave::start_vaultron;
#[tokio::main]
async fn main() {
    let result = start_vaultron().await;
    println!("Server exit: {:?}", result);
}
