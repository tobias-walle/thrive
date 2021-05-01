use thrive_server::{start, ServerAddress};

#[tokio::main]
async fn main() {
    let address = ServerAddress::from_port(10000);
    println!("Start Server on {}...", address);
    start(&address).await.unwrap();
}
