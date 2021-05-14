use std::{fmt, sync::Mutex};

use actix_web::{web, App, HttpServer};
use thrive_core::state::State;

mod static_files;
mod ws;

#[derive(Debug)]
pub struct ServerAddress {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl Default for ServerAddress {
    fn default() -> Self {
        ServerAddress {
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: 10000,
        }
    }
}

impl ServerAddress {
    pub fn from_port(port: u16) -> ServerAddress {
        ServerAddress {
            port,
            ..Default::default()
        }
    }
}

impl fmt::Display for ServerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}:{}", self.protocol, self.host, self.port)
    }
}

impl ServerAddress {
    pub fn full(&self) -> String {
        self.to_string()
    }

    pub fn without_protocol(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn find_available_address() -> ServerAddress {
    let port = portpicker::pick_unused_port().expect("Couldn't find a free port.");
    ServerAddress::from_port(port)
}

async fn start_server(address: &str) -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Mutex::new(State::new()))
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
