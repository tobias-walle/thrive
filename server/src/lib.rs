use actix_web::{web, App, HttpServer};
use derive_more::{Deref, DerefMut};
use std::sync::RwLock;
use thrive_core::state::State;

pub use server_address::*;

mod static_files;
mod ws;

pub mod server_address;

#[derive(Deref, DerefMut)]
pub struct ServerState(RwLock<State>);

impl ServerState {
    fn new() -> Self {
        Self(RwLock::new(State::new()))
    }
}

async fn start_server(address: &str) -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(ServerState::new())
            .route("/ws", web::get().to(ws::index))
            .service(static_files::service())
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

pub async fn start(address: &ServerAddress) -> anyhow::Result<()> {
    let address = address.without_protocol();
    // Wrap the actix runtime into the tokio runtime, so we can use both
    tokio::task::spawn_blocking(move || {
        let mut system = actix_web::rt::System::new("server");
        system.block_on(async move { start_server(&address).await })
    })
    .await??;
    Ok(())
}
