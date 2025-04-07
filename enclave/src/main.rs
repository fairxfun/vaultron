use vaultron_enclave::start_server;

#[tokio::main]
async fn main() {
    let result = start_server().await;
    println!("Server exit: {:?}", result);
}
