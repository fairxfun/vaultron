use enclave_tester::start_tester;

#[tokio::main]
async fn main() {
    let result = start_tester().await;
    println!("Tester exit: {:?}", result);
}
