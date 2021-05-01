use thrive_server::{find_available_address, start};

#[tokio::main]
async fn main() {
    let address = find_available_address();
    println!("Start Server on {}...", &address);
    start(&address).await.unwrap();
}
