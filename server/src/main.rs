use thrive_server::{start, ServerAddress};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let address = ServerAddress::from_port(10000);
    println!("Start Server on {}...", address);
    start(&address).await.unwrap();
}
