use vaultron_enclave::server::start_server;

#[tokio::main]
async fn main() {
    let result = start_server().await;
    println!("Server exit: {:?}", result);
}
