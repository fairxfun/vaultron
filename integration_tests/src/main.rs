use integration_tests::start_test;

#[tokio::main]
async fn main() {
    let result = start_test().await;
    println!("Tester exit: {:?}", result);
}
